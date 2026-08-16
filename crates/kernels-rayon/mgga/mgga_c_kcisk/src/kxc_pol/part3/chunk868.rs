//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 868/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk868(t13038: f64, t334: f64, t3688: f64, t45: f64, t1201: f64, t1213: f64, t12889: f64, t12893: f64, t12897: f64, t12902: f64, t12904: f64, t12907: f64, t12914: f64, t13018: f64, t13026: f64) -> (f64, f64) {
    let t13039 = t13038 * t334;
    let t13042 = t45 * t3688;
    let t13045 = -0.1025389702100779493e4_f64 * t1201 * t12889 - 0.51947267698127589897e2_f64 * t1201 * t12893 + 0.35089340384731224426e1_f64 * t1201 * t12897 + t12902 + t12904 + t12907 - t12914 + t13018 + t13026 + 0.19751789702565206229e-1_f64 * t45 * t13039 - 0.17544670192365612213e1_f64 * t13042 * t1213;
    (t13039, t13045)
}
