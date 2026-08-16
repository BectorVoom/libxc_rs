//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta27 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk183;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk184;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk185;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk186;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk187;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk188;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk189;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta27(t30: f64, t33: f64, t512: f64, t521: f64, t187: f64, t520: f64, t513: f64, t199: f64, t516: f64, zeta_threshold: f64, t136: f64, t221: f64, t149: f64, t225: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t522, t524, t525, t527, t530) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk183(t30, t33, t512, t521, t187, t520, t513, t199, t516, zeta_threshold);
        let t531 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk184(t530);
        let t532 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk185(t530, t531);
        let t533 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk186(t531);
        let (t535, t539) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk187(t136, t221, t533, t149, t225, t522, t524);
        let t540 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk188(t532);
        let t541 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk189(t540, t73);
    (t522, t524, t525, t527, t530, t531, t532, t533, t535, t539, t540, t541)
}
