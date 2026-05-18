//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1303/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1303<F: Float>(t1342: F, t2416: F, t49581: F, t13900: F, t4814: F, t7669: F, t56957: F, t57113: F, t57117: F, t57120: F, t57185: F, t57213: F, t57215: F, t57217: F, t57219: F, t57222: F) -> (F, F, F) {
    let t57225 = F::new(0.64327297288604419288e2) * t2416 * t49581 * t1342;
    let t57228 = F::new(0.3103500882342370105e4) * t7669 * t13900 * t4814;
    let t57229 = t56957 + t57113 + t57117 + t57120 - t57185 + t57213 + t57215 + t57217 + t57219 - t57222 + t57225 + t57228;
    (t57225, t57228, t57229)
}
