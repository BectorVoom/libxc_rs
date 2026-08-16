//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3011/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3011(t381: f64, t61719: f64, t1058: f64, t1060: f64, t11034: f64, t11046: f64, t11059: f64, t14488: f64, t14577: f64, t14630: f64, t14651: f64, t1629: f64, t18089: f64, t18100: f64, t18112: f64, t18139: f64, t18142: f64, t18151: f64, t3166: f64, t3180: f64, t3186: f64, t3188: f64, t3200: f64, t3201: f64, t43473: f64, t4678: f64, t5866: f64, t5932: f64, t5936: f64, t62945: f64) -> (f64, f64) {
    let t62984 = t381 * t61719;
    let t62988 = t1058 * t1060 * t3166 * t5866 - 2.0_f64 * t14488 * t1629 * t3200 * t3201 + 2.0_f64 * t11046 * t14630 * t5932 + 6.0_f64 * t11059 * t14577 * t5936 + 2.0_f64 * t3186 * t3188 * t62945 - 2.0_f64 * t3200 * t3201 * t62984 + 8.0_f64 * t11034 * t18139 + 8.0_f64 * t11034 * t18142 + 4.0_f64 * t14651 * t4678 + 4.0_f64 * t18089 * t3180 + 2.0_f64 * t18100 * t3180 + 12.0_f64 * t18112 * t43473 + 4.0_f64 * t18151 * t3180;
    (t62984, t62988)
}
