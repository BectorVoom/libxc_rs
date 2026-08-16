//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1962/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1962(t2470: f64, t28779: f64, t25895: f64, t102185: f64, t102205: f64, t102213: f64, t102217: f64, t1398: f64, t1444: f64, t1903: f64, t25924: f64, t26079: f64, t26333: f64, t26343: f64, t27837: f64, t28862: f64, t28888: f64, t4003: f64, t4056: f64, t543: f64, t7295: f64, t7296: f64, t7301: f64, t8085: f64, t96232: f64, t96234: f64, t96237: f64, t96240: f64) -> (f64, f64) {
    let t102218 = t28779 * t2470;
    let t102219 = t25895 * t102218;
    let t102222 = 0.8673628188205199462e0_f64 * t7295 * t7301 * t28888 * t1398 * t543 - 0.52041769129231196772e1_f64 * t7295 * t25924 * t28862 * t1444 - 0.8673628188205199462e0_f64 * t7295 * t26079 * t102185 * t4003 - 0.8673628188205199462e0_f64 * t27837 * t26343 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t102185 * t543 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t8085 * t4056 * t543 + 0.72280234901709995518e-2_f64 * t96232 + 0.25702851531048074406e-1_f64 * t96234 + 0.45699670022203476294e-2_f64 * t102205 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t26333 * t1903 - 0.51405703062096148812e-1_f64 * t96237 + t102213 - t102217 + 0.19274729307122665472e-1_f64 * t102219 + 0.51405703062096148812e-1_f64 * t96240;
    (t102218, t102222)
}
