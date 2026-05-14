//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1170/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1170<F: Float>(t56957: F, t57113: F, t57117: F, t57120: F, t57185: F, t57213: F, t57215: F, t57217: F, t57219: F, t57222: F, t57225: F, t57228: F, t13890: F, t4818: F, t7681: F, t3780: F, t49939: F, t845: F) -> (F, F, F) {
    let t57229 = t56957 + t57113 + t57117 + t57120 - t57185 + t57213 + t57215 + t57217 + t57219 - t57222 + t57225 + t57228;
    let t57233 = 0.57894567559743977359e3 * t7681 * t13890 * t4818;
    let t57236 = 0.69263023597503453196e2 * t845 * t49939 * t3780;
    (t57229, t57233, t57236)
}
