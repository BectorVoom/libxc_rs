//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1957/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1957(t28780: f64, t94886: f64, t28889: f64, t686: f64, t72: f64, t7284: f64, t10073: f64, t1903: f64, t2102: f64, t25929: f64, t28837: f64, t3920: f64) -> (f64, f64, f64, f64, f64) {
    let t102113 = 0.51405703062096148812e-1_f64 * t94886 * t28780;
    let t102115 = t28889 * t72 * t686;
    let t102117 = 0.14456046980341999104e-1_f64 * t7284 * t102115;
    let t102120 = t10073 * t25929 * t2102 * t1903;
    let t102122 = t28837 * t3920;
    (t102113, t102115, t102117, t102120, t102122)
}
