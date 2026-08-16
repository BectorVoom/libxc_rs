//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1698/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1698(t23714: f64, t4724: f64, t981: f64, t4711: f64, t78429: f64, t23446: f64, t4719: f64, t23453: f64, t19049: f64, t6219: f64, t88510: f64, t88562: f64, t88564: f64, t88567: f64, t88600: f64, t88602: f64, t88607: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88986 = 0.46785788981077169656e1_f64 * t981 * t4724 * t23714;
    let t88989 = 0.69263436422725855036e2_f64 * t981 * t78429 * t4711;
    let t88991 = 0.14035736694323150897e2_f64 * t4719 * t23446;
    let t88993 = 0.4155806185363551302e3_f64 * t4719 * t23453;
    let t88995 = 0.70178683471615754484e1_f64 * t19049 * t6219;
    let t88996 = t88600 - t88602 + t88510 - t88607 - t88562 + t88564 - t88567 + t88986 - t88989 + t88991 + t88993 + t88995;
    (t88986, t88989, t88991, t88993, t88995, t88996)
}
