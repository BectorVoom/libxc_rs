//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2190/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2190(t27989: f64, t98380: f64, t689: f64, t6919: f64, t7242: f64, t1904: f64, t2022: f64, t22386: f64, t25924: f64, t27868: f64, t27980: f64, t28008: f64, t6895: f64, t7274: f64, t7295: f64, t7296: f64, t75188: f64, t75267: f64, t7930: f64, t94409: f64, t94580: f64, t94591: f64, t94593: f64, t97719: f64, t97734: f64, t98056: f64) -> f64 {
    let t108153 = t98380 * t27989;
    let t108156 = t689 * t7242 * t6919;
    let t108172 = t97719 - t94409 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t2022 * t22386 - t97734 - 0.13170898365871023197e1_f64 * t98056 * t1904 + 0.65049603595885220126e-3_f64 * t94580 + 0.25702851531048074406e-1_f64 * t108153 + 0.54878743191129263322e-2_f64 * t108156 - 0.8673628188205199462e0_f64 * t28008 * t7930 - 0.8673628188205199462e0_f64 * t27868 * t27980 * t75267 + 0.45699670022203476294e-2_f64 * t94591 - 0.17347256376410398924e1_f64 * t27868 * t27980 * t75188 - 0.26020884564615598386e1_f64 * t7295 * t25924 * t7274 * t6895 + 0.17135234354032049604e-1_f64 * t94593;
    t108172
}
