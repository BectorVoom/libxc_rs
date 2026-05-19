//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 995/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk995<F: Float>(t9229: F, t9240: F, t987: F, t1414: F, t2537: F, t1426: F, t2598: F, t2576: F, t1396: F, t2477: F, t2480: F, t9011: F) -> (F, F, F, F, F, F, F, F) {
    let t9241 = t9229 + t9240;
    let t9242 = t9241 * t987;
    let t9245 = t1414 * t2537;
    let t9248 = t1426 * t2598;
    let t9255 = t1426 * t2576;
    let t9258 = t1396 * t2477;
    let t9260 = F::new(2.0) * t9258 * t2480;
    let t9264 = F::cast_from(0.35616666666666666666e-1_f64) * t9011;
    (t9241, t9242, t9245, t9248, t9255, t9258, t9260, t9264)
}
