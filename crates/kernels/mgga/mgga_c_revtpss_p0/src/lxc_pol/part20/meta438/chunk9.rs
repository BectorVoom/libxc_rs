//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1660/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1660<F: Float>(t12361: F, t12411: F, t12547: F, t3523: F, t1168: F, t1187: F, t1189: F, t12423: F, t12429: F, t12464: F, t12465: F, t12470: F, t12472: F, t12481: F, t12486: F, t12491: F, t12497: F, t12501: F, t12508: F, t12553: F, t3452: F, t3453: F, t3454: F, t3471: F, t3477: F, t3479: F, t3480: F, t3496: F, t3497: F, t3498: F, t3515: F, t3521: F, t3524: F, t43977: F, t45043: F, t45045: F, t45048: F, t45050: F) -> (F, F) {
    let t45282 = F::new(24.0) * t12361 * t12411;
    let t45289 = t12547 * t3523;
    let t45293 = F::cast_from(0.3859675079686208416e3_f64) * t12423 * t12508 + F::cast_from(0.61524113149298439947e4_f64) * t12553 * t43977 * t3497 + F::new(36.0) * t3477 * t3454 * t3471 - t45043 - t45045 - t45048 - t45050 - F::cast_from(0.11579025239058625248e4_f64) * t12429 * t3480 * t3471 - F::new(8.0) * t3452 * t12465 * t1168 + F::cast_from(0.12865583598954028054e3_f64) * t3477 * t12464 * t3479 * t1168 + F::cast_from(0.12414243100625616072e5_f64) * t12470 * t3453 * t12472 * t3471 - F::cast_from(0.14035736694323150897e2_f64) * t12491 * t12497 + F::cast_from(0.20779030926817756511e3_f64) * t12481 * t12501 - F::cast_from(0.62337092780453269531e3_f64) * t12486 * t3524 * t3515 + t45282 + F::cast_from(0.21053605041484726346e2_f64) * t3521 * t3498 * t3515 - F::cast_from(0.46785788981077169656e1_f64) * t3496 * t1189 * t12547 + F::cast_from(0.69263436422725855036e2_f64) * t3521 * t45289 * t1187;
    (t45282, t45293)
}
