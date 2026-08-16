//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1245/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1245(t2724: f64, t4364: f64, t4365: f64, t2482: f64, t2719: f64, t814: f64, t14671: f64, t14686: f64, t4366: f64, t10891: f64, t10893: f64, t10906: f64, t14894: f64, t14896: f64, t14900: f64, t14904: f64, t14907: f64, t14910: f64, t14914: f64, t14919: f64, t14925: f64, t2745: f64, t4362: f64) -> f64 {
    let t14927 = t4364 * t4365 * t2724;
    let t14931 = t2482 * t2719 * t814;
    let t14933 = t14686 * t14671 * t4366;
    let t14934 = t14931 * t14933;
    let t14936 = -35.0_f64 / 108.0_f64 * t10891 + 7.0_f64 / 144.0_f64 * t10893 - 7.0_f64 / 48.0_f64 * t10906 - 0.12862205435420921092e-2_f64 * t14894 * t14896 - 0.17149607247227894789e-2_f64 * t4362 * t14900 + 0.85748036236139473944e-3_f64 * t4362 * t14904 - 0.80031500487063509014e-2_f64 * t14907 + 0.85748036236139473944e-3_f64 * t2745 * t14910 - 0.21437009059034868486e-3_f64 * t2745 * t14914 - 0.42874018118069736972e-2_f64 * t2745 * t14919 - t14925 + 0.12862205435420921092e-2_f64 * t4362 * t14927 + 0.50820002809285328225e-4_f64 * t14934;
    t14936
}
