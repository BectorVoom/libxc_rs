//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 972/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk972<F: Float>(t1123: F, t5728: F, t2027: F, t287: F, t302: F, t2739: F, t759: F, t761: F, t2105: F, t1066: F, t2009: F, t2031: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7665 = t1123 * t5728;
    let t7666 = t2027 * t287;
    let t7667 = t7665 * t7666;
    let t7668 = t302 * t7667;
    let t7671 = t2739 * t759;
    let t7672 = t7671 * t761;
    let t7673 = t2105 * t7672;
    let t7676 = t1066 * t2009;
    let t7677 = t7676 * t761;
    let t7678 = t2105 * t7677;
    let t7681 = t1066 * t2027;
    let t7682 = t7681 * t2031;
    (t7665, t7666, t7667, t7668, t7672, t7673, t7677, t7678, t7681, t7682)
}
