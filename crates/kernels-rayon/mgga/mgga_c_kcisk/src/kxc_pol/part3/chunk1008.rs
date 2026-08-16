//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1008/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1008(t240: f64, t3688: f64, t1213: f64, t12889: f64, t12893: f64, t13050: f64, t13053: f64, t13056: f64, t13060: f64, t13066: f64, t13105: f64, t13123: f64, t1550: f64, t3718: f64, t3726: f64, t4486: f64) -> f64 {
    let t14850 = t240 * t3688;
    let t14865 = -0.17544670192365612213e1_f64 * t14850 * t1213 - 0.17544670192365612213e1_f64 * t4486 * t3718 - 0.1025389702100779493e4_f64 * t1550 * t12889 - 0.35089340384731224426e1_f64 * t1550 * t13105 - t13050 + t13053 - t13056 + t13060 - 0.51947267698127589897e2_f64 * t1550 * t12893 - 0.51947267698127589899e2_f64 * t4486 * t3726 + 0.1038945353962551798e3_f64 * t1550 * t13066 - t13123;
    t14865
}
