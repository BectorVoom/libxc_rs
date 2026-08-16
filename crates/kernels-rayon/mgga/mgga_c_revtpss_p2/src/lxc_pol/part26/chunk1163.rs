//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1163/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1163(t114: f64, t2327: f64, t7356: f64, t94973: f64, t94976: f64, t94979: f64, t94981: f64, t94983: f64, t94986: f64, t94988: f64, t10259: f64, t10416: f64, t1312: f64, t13435: f64, t13440: f64, t2055: f64, t2322: f64, t2371: f64, t26153: f64, t26399: f64, t28658: f64, t46126: f64, t49693: f64, t49851: f64, t5523: f64, t60551: f64, t670: f64, t7359: f64, t7373: f64, t95347: f64, t95357: f64) -> (f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t95371 = t7356 * t2327;
    let t95397 = 308.0_f64 / 27.0_f64 * t94973;
    let t95405 = piecewise3(t115, 0.0_f64, -t95397 - 22.0_f64 / 3.0_f64 * t94976 - 4.0_f64 * t94979 + 2.0_f64 * t94981 - 3.0_f64 / 2.0_f64 * t94983 + 3.0_f64 / 2.0_f64 * t94986 - t94988 / 4.0_f64);
    let t95408 = 2.0_f64 * t10259 * t7359 + 6.0_f64 * t10416 * t7373 + 2.0_f64 * t1312 * t95405 + 12.0_f64 * t13435 * t7373 + 6.0_f64 * t13440 * t7373 + 2.0_f64 * t2055 * t46126 + 6.0_f64 * t2055 * t49693 + 6.0_f64 * t2055 * t49851 + 2.0_f64 * t2055 * t60551 + 6.0_f64 * t2322 * t26153 + 6.0_f64 * t2371 * t26399 + 6.0_f64 * t2371 * t28658 + 6.0_f64 * t26153 * t5523 + 6.0_f64 * t670 * t95357 + t95347 + 6.0_f64 * t95371;
    (t95371, t95405, t95408)
}
