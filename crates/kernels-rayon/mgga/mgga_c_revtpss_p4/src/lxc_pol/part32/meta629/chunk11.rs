//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2030/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2030(t18805: f64, t95936: f64, t103037: f64, t103424: f64, t106143: f64, t106360: f64, t106365: f64, t110493: f64, t14495: f64, t231: f64, t25391: f64, t26547: f64, t26550: f64, t27199: f64, t27353: f64, t28405: f64, t6072: f64, t7070: f64, t7076: f64, t93349: f64, t95911: f64, t95914: f64, t95925: f64, t95927: f64, t95930: f64) -> f64 {
    let t110639 = t95936 * t18805;
    let t110665 = -0.65854491829355115987e0_f64 * t26547 * t6072 + 0.19514881078765566037e-1_f64 * t110639 + 0.48186823267806663678e-3_f64 * t95911 + t95914 - 0.17347256376410398924e1_f64 * t25391 * t103037 * t14495 + 0.65049603595885220126e-3_f64 * t95925 - 0.13009920719177044025e-1_f64 * t95927 - t95930 - 0.8673628188205199462e0_f64 * t25391 * t26550 * t106143 + 0.26020884564615598386e1_f64 * t93349 * t26550 * t106360 + 0.8673628188205199462e0_f64 * t27353 * t103424 * t14495 - 0.8673628188205199462e0_f64 * t25391 * t26550 * t106365 + 0.8673628188205199462e0_f64 * t27199 * t28405 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t110493 * t231;
    t110665
}
