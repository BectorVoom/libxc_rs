//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1146/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1146<F: Float>(t1012: F, t1014: F, t1017: F, t1030: F, t3053: F, t30840: F, t3068: F, t30827: F, t23448: F, t8384: F, t23442: F, t1036: F, t30833: F) -> (F, F, F, F, F, F) {
    let t113397 = t1012 * t1014 * t1030 * t1017;
    let t113400 = t30840 * t3053;
    let t113413 = t1012 * t30827 * t3068;
    let t113416 = t23448 * t8384;
    let t113418 = t23442 * t8384;
    let t113429 = t30833 * t1036;
    (t113397, t113400, t113413, t113416, t113418, t113429)
}
