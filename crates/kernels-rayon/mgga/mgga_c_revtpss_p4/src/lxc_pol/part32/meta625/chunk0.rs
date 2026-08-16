//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1978/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1978(t28780: f64, t97700: f64, t6861: f64, t7506: f64, t1364: f64, t30248: f64, t786: f64, t102329: f64, t102339: f64, t102346: f64, t102661: f64, t108206: f64, t1444: f64, t2097: f64, t22252: f64, t25930: f64, t26079: f64, t26304: f64, t27837: f64, t27864: f64, t28863: f64, t30071: f64, t30247: f64, t4003: f64, t543: f64, t7295: f64, t7296: f64, t7301: f64, t7532: f64, t94823: f64, t96380: f64, t96382: f64) -> (f64, f64) {
    let t109567 = t97700 * t28780;
    let t109573 = t7506 * t6861;
    let t109579 = t786 * t30248 * t1364;
    let t109598 = -0.28912093960683998207e-1_f64 * t109567 - t102329 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t30247 * t1444 - t102339 - 0.8673628188205199462e0_f64 * t7295 * t26079 * t109573 * t4003 + 0.9757440539382783019e-2_f64 * t109579 - 0.8673628188205199462e0_f64 * t25930 * t26304 * t108206 - t102346 + 0.52041769129231196772e1_f64 * t94823 * t102661 * t27864 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t2097 * t22252 * t543 + 0.17135234354032049604e-2_f64 * t96380 + 0.17135234354032049604e-2_f64 * t96382 - 0.4336814094102599731e0_f64 * t30071 * t7532 + 0.17347256376410398924e1_f64 * t27837 * t28863;
    (t109573, t109598)
}
