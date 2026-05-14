//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 518/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk518<F: Float>(t1585: F, t4215: F, t1483: F, t2916: F, t1218: F, t1497: F, t1217: F) -> (F, F, F, F) {
    let t4216 = t1585 * t4215;
    let t4224 = t2916 * t1483;
    let t4229 = t1218 * t1497;
    let t4230 = t1217 * t4229;
    (t4216, t4224, t4229, t4230)
}
