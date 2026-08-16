//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 907/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk907(t25392: f64, t27349: f64, t1955: f64, t7057: f64, t14495: f64, t1949: f64, t2718: f64, t14587: f64, t25383: f64, t25388: f64, t25391: f64, t25400: f64, t25406: f64, t25414: f64, t25424: f64, t25432: f64, t27335: f64, t27338: f64, t27342: f64, t27344: f64, t7083: f64, t7766: f64, t7770: f64) -> (f64, f64, f64, f64, f64) {
    let t27350 = t25392 * t27349;
    let t27353 = t1955 * t7057;
    let t27354 = t25392 * t14495;
    let t27357 = t2718 * t1949;
    let t27358 = t27357 * t14587;
    let t27361 = 0.25702851531048074406e-1_f64 * t25388 - 0.9757440539382783019e-2_f64 * t25400 - 0.4336814094102599731e0_f64 * t7766 * t7083 - t25406 + 0.54878743191129263322e-2_f64 * t27335 + 0.12851425765524037203e-1_f64 * t25414 + 0.72280234901709995518e-2_f64 * t27338 + t25424 - 0.14456046980341999104e-1_f64 * t27342 + 0.25702851531048074406e-1_f64 * t27344 + 0.8673628188205199462e0_f64 * t25383 * t7770 - 0.72280234901709995518e-2_f64 * t25432 - 0.8673628188205199462e0_f64 * t25391 * t27350 + 0.4336814094102599731e0_f64 * t27353 * t27354 - 0.8673628188205199462e0_f64 * t27353 * t27358;
    (t27350, t27353, t27354, t27358, t27361)
}
