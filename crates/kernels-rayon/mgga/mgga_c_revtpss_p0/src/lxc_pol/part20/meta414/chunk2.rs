//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1531/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1531(t11651: f64, t11659: f64, t11774: f64, t11776: f64, t11866: f64, t11871: f64, t16025: f64, t3096: f64, t3117: f64, t3120: f64, t372: f64, t42315: f64, t43029: f64, t43032: f64, t43035: f64, t43038: f64, t43044: f64, t43050: f64, t43051: f64, t43057: f64, t43063: f64, t43066: f64, t43069: f64) -> f64 {
    let t43074 = -t43029 / 36.0_f64 + t43032 / 54.0_f64 - 0.17149607247227894789e-2_f64 * t43035 - 0.25724410870841842184e-2_f64 * t43038 * t3120 - 0.25724410870841842184e-2_f64 * t11866 * t11871 - 0.25724410870841842184e-2_f64 * t43044 * t3117 * t11659 * t16025 + 0.51448821741683684368e-2_f64 * t43050 * t3117 * t11659 * t43051 - 0.28582678745379824648e-2_f64 * t11774 * t42315 * t43057 - 0.22866142996303859719e-2_f64 * t43063 + 0.18292914397043087775e-1_f64 * t43066 * t11776 + 0.34299214494455789578e-2_f64 * t43069 * t372 * t11651 * t3096;
    t43074
}
