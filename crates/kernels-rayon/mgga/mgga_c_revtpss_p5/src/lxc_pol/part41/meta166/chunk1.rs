//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 711/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk711(t4533: f64, t868: f64, t1580: f64, t213: f64, t2437: f64, t2443: f64, t2446: f64, t2449: f64, t2460: f64, t2462: f64, t2468: f64, t2473: f64, t257: f64, t2765: f64, t4323: f64, t4326: f64, t4470: f64, t4474: f64, t4478: f64, t4482: f64, t4487: f64, t865: f64, t887: f64) -> (f64, f64) {
    let t4534 = t868 * t4533;
    let t4537 = t2437 - t2443 - 0.54878743191129263322e-2_f64 * t2446 + 0.54878743191129263322e-2_f64 * t2449 + t2460 + 0.9757440539382783019e-2_f64 * t2462 - 0.9757440539382783019e-2_f64 * t2468 - t2473 - 0.54878743191129263322e-2_f64 * t4323 + 0.9757440539382783019e-2_f64 * t4326 + 0.65854491829355115987e0_f64 * t213 * t4470 * t257 - 0.65854491829355115987e0_f64 * t4474 * t887 + 0.54878743191129263322e-2_f64 * t4478 - 0.9757440539382783019e-2_f64 * t4482 - 0.65854491829355115987e0_f64 * t2765 * t1580 + 0.13170898365871023197e1_f64 * t865 * t4487 - 0.65854491829355115987e0_f64 * t865 * t4534;
    (t4534, t4537)
}
