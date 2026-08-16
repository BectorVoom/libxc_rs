//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 879/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk879(t119: f64, t2379: f64, t10: f64, t2378: f64, t88: f64, t975: f64, t2340: f64, t747: f64, t106: f64, t2393: f64, t1011: f64, t1052: f64, t2210: f64, t2214: f64, t2394: f64, t4098: f64, t4346: f64, t4348: f64, t709: f64, t713: f64, t7706: f64, t7768: f64, t7776: f64, t7962: f64, t98: f64) -> (f64, f64) {
    let t9183 = t2379 * t119;
    let t9187 = t2378 * t88 * t10;
    let t9188 = t975 * t9187;
    let t9204 = t747 * t2340;
    let t9205 = t106 * t9204;
    let t9209 = t2393 * t119;
    let t9217 = t9183 * t709 / 6.0_f64 - t9188 * t98 / 6.0_f64 + t4098 * t2210 / 6.0_f64 + t1052 * t7962 / 6.0_f64 + t1052 * t7768 / 6.0_f64 + t4098 * t2214 / 6.0_f64 + t1052 * t7776 / 6.0_f64 + t1052 * t7706 / 6.0_f64 + t4346 / 9.0_f64 + t9205 / 9.0_f64 + t9183 * t713 / 6.0_f64 + t9209 * t713 / 6.0_f64 + t9209 * t709 / 6.0_f64 - t4348 / 9.0_f64 - t2394 * t1011 / 6.0_f64;
    (t9204, t9217)
}
