//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 892/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk892(t2840: f64, t287: f64, t275: f64, t10294: f64, t10544: f64, t891: f64, t2843: f64, t290: f64, t2860: f64, t919: f64, t2904: f64, t938: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10660 = 1.0_f64 / t2840 / t287;
    let t10661 = t275 * t10660;
    let t10675 = 0.36514074074074074075e0_f64 * t10294;
    let t10676 = 0.93011851851851851854e0_f64 * t10544;
    let t10701 = 1.0_f64 / t2840 / t891;
    let t10702 = t275 * t10701;
    let t10704 = 1.0_f64 / t2843 / t290;
    let t10740 = t919 * t2860;
    let t10747 = t938 * t2904;
    (t10661, t10675, t10676, t10702, t10704, t10740, t10747)
}
