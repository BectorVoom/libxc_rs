//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1297/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1297<F: Float>(t1995: F, t9242: F, t2860: F, t7228: F, t237: F, t9462: F, t732: F, t1116: F, t20663: F, t7555: F, t2866: F, t7560: F, t7532: F, t1987: F, t9398: F, t25622: F, t25626: F, t25644: F, t25647: F, t25651: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25653 = 0.5848223622634646207e0 * t9242 * t1995;
    let t25655 = 0.20508037716432813315e4 * t2860 * t7228;
    let t25656 = t237 * t9462;
    let t25658 = 0.11696447245269292414e1 * t25656 * t732;
    let t25660 = 0.11696447245269292414e1 * t20663 * t1116;
    let t25662 = 0.23392894490538584828e1 * t2860 * t7555;
    let t25664 = 0.46785788981077169656e1 * t7560 * t2866;
    let t25666 = 0.2077903092681775651e3 * t2860 * t7532;
    let t25668 = 0.2077903092681775651e3 * t1987 * t9398;
    let t25669 = t25622 + t25626 + t25644 + t25647 + t25651 - t25653 - t25655 - t25658 - t25660 + t25662 + t25664 + t25666 + t25668;
    (t25653, t25655, t25658, t25660, t25662, t25664, t25666, t25668, t25669)
}
