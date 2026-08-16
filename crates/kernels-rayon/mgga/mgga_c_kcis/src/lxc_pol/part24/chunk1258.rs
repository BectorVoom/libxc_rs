//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1258/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1258(t303: f64, t6487: f64, t92972: f64, t1014: f64, t29003: f64, t1094: f64, t6481: f64, t1122: f64, t19541: f64, t2179: f64, t28961: f64, t100497: f64, t100501: f64, t26960: f64, t92964: f64, t96000: f64, t97166: f64, t97173: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100558 = t303 * t92972 * t6487;
    let t100566 = t1014 * t29003;
    let t100568 = t6481 * t1094;
    let t100570 = t303 * t100568 * t1122;
    let t100573 = t303 * t19541 * t2179;
    let t100575 = t1014 * t28961;
    let t100577 = 0.34822083333333333332e-2_f64 * t100558 + 0.15445601851851851852e-3_f64 * t97166 - 0.23168402777777777778e-3_f64 * t26960 * t100497 - 0.46336805555555555556e-3_f64 * t26960 * t100501 + 0.77382407407407407407e-3_f64 * t96000 + t97173 - t92964 + 0.23214722222222222221e-2_f64 * t100566 - 0.17411041666666666666e-2_f64 * t100570 + 0.17411041666666666666e-2_f64 * t100573 - 0.23214722222222222221e-2_f64 * t100575;
    (t100558, t100566, t100570, t100573, t100575, t100577)
}
