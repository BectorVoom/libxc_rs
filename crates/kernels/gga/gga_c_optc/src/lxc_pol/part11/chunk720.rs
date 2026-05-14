//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 720/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk720<F: Float>(t1313: F, t193: F, t6654: F, t1320: F, t2229: F, t1326: F, t1773: F) -> (F, F, F) {
    let t10048 = t193 * t6654 * t1313;
    let t10079 = t2229 * t1320;
    let t10188 = t1773 * t1326;
    (t10048, t10079, t10188)
}
