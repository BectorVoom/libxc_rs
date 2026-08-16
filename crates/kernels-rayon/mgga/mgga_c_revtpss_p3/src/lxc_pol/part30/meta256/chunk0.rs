//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1121/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1121(t213: f64, t5744: f64, t4086: f64, t1892: f64, t545: f64, t869: f64, t689: f64, t72: f64, t1432: f64, t686: f64, t1385: f64, t1399: f64, t1437: f64, t1883: f64, t4082: f64, t4085: f64, t4090: f64, t4094: f64, t4099: f64, t4105: f64, t4109: f64, t4113: f64, t4118: f64, t546: f64, t5659: f64, t5675: f64, t5710: f64, t5735: f64, t5738: f64, t5742: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5745 = t213 * t5744;
    let t5755 = t213 * t4086;
    let t5759 = t545 * t1892;
    let t5760 = t869 * t5759;
    let t5761 = t689 * t5760;
    let t5763 = t1892 * t72;
    let t5765 = t1432 * t5763 * t686;
    let t5767 = t1385 * t1892;
    let t5774 = t4082 - t4085 + 0.54878743191129263322e-2_f64 * t4090 - 0.54878743191129263322e-2_f64 * t4094 + t4099 - 0.9757440539382783019e-2_f64 * t4105 + 0.9757440539382783019e-2_f64 * t4109 - t4113 + 0.54878743191129263322e-2_f64 * t5738 - 0.9757440539382783019e-2_f64 * t5742 + 0.13170898365871023197e1_f64 * t5745 * t5735 * t5675 - 0.65854491829355115987e0_f64 * t820 * t4118 * t1883 - 0.65854491829355115987e0_f64 * t820 * t1437 * t5659 - 0.65854491829355115987e0_f64 * t5755 * t5735 * t1399 - 0.54878743191129263322e-2_f64 * t5761 + 0.9757440539382783019e-2_f64 * t5765 - 0.65854491829355115987e0_f64 * t820 * t5767 * t1399 + 0.65854491829355115987e0_f64 * t213 * t546 * t5710;
    (t5745, t5755, t5759, t5760, t5763, t5767, t5774)
}
