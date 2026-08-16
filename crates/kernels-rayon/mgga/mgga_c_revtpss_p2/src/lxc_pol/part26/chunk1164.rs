//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1164/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1164(t10260: f64, t10415: f64, t10416: f64, t13207: f64, t13216: f64, t2055: f64, t2089: f64, t2322: f64, t2372: f64, t25082: f64, t25188: f64, t26154: f64, t26218: f64, t26383: f64, t26405: f64, t26412: f64, t26679: f64, t28167: f64, t28196: f64, t28286: f64, t28658: f64, t3813: f64, t4254: f64, t49630: f64, t49654: f64, t508: f64, t569: f64, t651: f64, t7235: f64, t7357: f64, t7359: f64, t7374: f64, t7378: f64, t7539: f64, t9069: f64, t95405: f64, t95408: f64, t9984: f64) -> f64 {
    let t95446 = 6.0_f64 * t28196 * t28286 * t49654 - 9.0_f64 * t25082 * t26405 * t49630 + t95408 * t569 + 18.0_f64 * t7235 * t26412 + 9.0_f64 * t7235 * t26383 - 2.0_f64 * t651 * t508 * t95405 - 6.0_f64 * t10416 * t7378 - 3.0_f64 * t7357 * t3813 - 3.0_f64 * t25188 * t7539 - 6.0_f64 * t28658 * t2372 - 6.0_f64 * t7359 * t13216 - 2.0_f64 * t7359 * t10260 - 6.0_f64 * t10416 * t7374 - 6.0_f64 * t4254 * t26154 - 6.0_f64 * t2322 * t26218 - 6.0_f64 * t4254 * t26218 - 2.0_f64 * t651 * t13207 * t2055 + 18.0_f64 * t28167 * t9069 * t9984 + 6.0_f64 * t7235 * t26679 - t10415 * t2089;
    t95446
}
