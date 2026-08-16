//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 933/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk933(t1424: f64, t1445: f64, t1904: f64, t213: f64, t3894: f64, t3898: f64, t3901: f64, t3904: f64, t3910: f64, t3912: f64, t3918: f64, t3922: f64, t4071: f64, t5601: f64, t5604: f64, t561: f64, t5711: f64, t5715: f64, t5719: f64, t5723: f64, t5728: f64, t5775: f64) -> f64 {
    let t5778 = t3894 - t3898 - 0.54878743191129263322e-2_f64 * t3901 + 0.54878743191129263322e-2_f64 * t3904 + t3910 + 0.9757440539382783019e-2_f64 * t3912 - 0.9757440539382783019e-2_f64 * t3918 - t3922 - 0.54878743191129263322e-2_f64 * t5601 + 0.9757440539382783019e-2_f64 * t5604 + 0.65854491829355115987e0_f64 * t213 * t5711 * t561 - 0.65854491829355115987e0_f64 * t5715 * t1445 + 0.54878743191129263322e-2_f64 * t5719 - 0.9757440539382783019e-2_f64 * t5723 - 0.65854491829355115987e0_f64 * t4071 * t1904 + 0.13170898365871023197e1_f64 * t1424 * t5728 - 0.65854491829355115987e0_f64 * t1424 * t5775;
    t5778
}
