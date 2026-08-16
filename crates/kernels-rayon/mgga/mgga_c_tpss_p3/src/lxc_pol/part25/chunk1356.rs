//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1356/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1356(t18006: f64, t18770: f64, t19736: f64, t19762: f64, t19767: f64, t19781: f64, t20463: f64, t20466: f64, t20482: f64, t20494: f64, t21312: f64, t21627: f64, t3721: f64, t5571: f64, t5572: f64, t61222: f64, t61226: f64, t62671: f64, t6337: f64, t64060: f64, t66362: f64, t66559: f64, t70030: f64, t70039: f64, t70042: f64, t70046: f64, t70060: f64, t70063: f64, t70074: f64, t70094: f64, t70103: f64, t70113: f64, t70123: f64, t70130: f64, t70134: f64) -> f64 {
    let t72026 = 2.0_f64 * t19767 * t66362 * t19781 - 2.0_f64 * t18006 * t18770 * t70123 + 6.0_f64 * t19767 * t66559 * t70042 - 6.0_f64 * t19767 * t20482 * t70046 - 4.0_f64 * t18006 * t66362 * t19762 - 4.0_f64 * t64060 * t20466 + 4.0_f64 * t5571 * t5572 * t6337 * t3721 + t19767 * t18770 * t70060 - 2.0_f64 * t18006 * t18770 * t70063 + 2.0_f64 * t19767 * t18770 * t70130 + t19767 * t18770 * t70134 + 6.0_f64 * t61226 * t18770 * t70103 - 4.0_f64 * t61222 * t21627 - 4.0_f64 * t18006 * t62671 * t21312 - 4.0_f64 * t18006 * t18770 * t70113 + 2.0_f64 * t70039 * t20494 - 4.0_f64 * t18006 * t18770 * t70030 + 4.0_f64 * t18006 * t20482 * t70074 - 4.0_f64 * t19767 * t20482 * t70094 - 12.0_f64 * t19736 * t20463;
    t72026
}
