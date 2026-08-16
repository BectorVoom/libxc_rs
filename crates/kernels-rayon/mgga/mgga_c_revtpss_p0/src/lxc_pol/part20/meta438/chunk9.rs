//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1660/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1660(t12361: f64, t12411: f64, t12547: f64, t3523: f64, t1168: f64, t1187: f64, t1189: f64, t12423: f64, t12429: f64, t12464: f64, t12465: f64, t12470: f64, t12472: f64, t12481: f64, t12486: f64, t12491: f64, t12497: f64, t12501: f64, t12508: f64, t12553: f64, t3452: f64, t3453: f64, t3454: f64, t3471: f64, t3477: f64, t3479: f64, t3480: f64, t3496: f64, t3497: f64, t3498: f64, t3515: f64, t3521: f64, t3524: f64, t43977: f64, t45043: f64, t45045: f64, t45048: f64, t45050: f64) -> (f64, f64) {
    let t45282 = 24.0_f64 * t12361 * t12411;
    let t45289 = t12547 * t3523;
    let t45293 = 0.3859675079686208416e3_f64 * t12423 * t12508 + 0.61524113149298439947e4_f64 * t12553 * t43977 * t3497 + 36.0_f64 * t3477 * t3454 * t3471 - t45043 - t45045 - t45048 - t45050 - 0.11579025239058625248e4_f64 * t12429 * t3480 * t3471 - 8.0_f64 * t3452 * t12465 * t1168 + 0.12865583598954028054e3_f64 * t3477 * t12464 * t3479 * t1168 + 0.12414243100625616072e5_f64 * t12470 * t3453 * t12472 * t3471 - 0.14035736694323150897e2_f64 * t12491 * t12497 + 0.20779030926817756511e3_f64 * t12481 * t12501 - 0.62337092780453269531e3_f64 * t12486 * t3524 * t3515 + t45282 + 0.21053605041484726346e2_f64 * t3521 * t3498 * t3515 - 0.46785788981077169656e1_f64 * t3496 * t1189 * t12547 + 0.69263436422725855036e2_f64 * t3521 * t45289 * t1187;
    (t45282, t45293)
}
