//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1934/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1934(t27960: f64, t545: f64, t2028: f64, t1904: f64, t2027: f64, t2030: f64, t26062: f64, t26065: f64, t26067: f64, t26071: f64, t26073: f64, t26084: f64, t27987: f64, t27990: f64, t27992: f64, t28003: f64, t28008: f64, t5728: f64, t7279: f64, t7292: f64, t7295: f64, t7308: f64, t7917: f64, t7930: f64) -> (f64, f64, f64) {
    let t28011 = t545 * t27960;
    let t28012 = t2028 * t28011;
    let t28017 = -0.54878743191129263322e-2_f64 * t27987 - 0.72280234901709995518e-2_f64 * t27990 + 0.12851425765524037203e-1_f64 * t27992 + 0.13170898365871023197e1_f64 * t7279 * t5728 - 0.65854491829355115987e0_f64 * t26084 * t1904 + 0.54878743191129263322e-2_f64 * t26062 + 0.9757440539382783019e-2_f64 * t26065 - 0.12851425765524037203e-1_f64 * t26067 - t26071 + 0.72280234901709995518e-2_f64 * t26073 + 0.8673628188205199462e0_f64 * t7295 * t28003 - 0.4336814094102599731e0_f64 * t7917 * t7308 - 0.4336814094102599731e0_f64 * t28008 * t2030 - 0.4336814094102599731e0_f64 * t2027 * t28012 - 0.4336814094102599731e0_f64 * t7292 * t7930;
    (t28011, t28012, t28017)
}
