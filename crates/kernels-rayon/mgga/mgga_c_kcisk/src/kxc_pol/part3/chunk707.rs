//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 707/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk707(t1705: f64, t4850: f64, t10894: f64, t10898: f64, t10907: f64, t10915: f64, t10918: f64, t10925: f64, t10929: f64, t10968: f64, t1726: f64, t1735: f64, t1747: f64, t45: f64, t4924: f64, t4931: f64, t4950: f64, t4958: f64, t634: f64) -> f64 {
    let t10972 = t4850 * t1705;
    let t10975 = 0.35089340384731224426e1_f64 * t4924 * t4931 + 0.35089340384731224426e1_f64 * t1735 * t10894 - 0.51947267698127589897e2_f64 * t1735 * t10898 - 0.1025389702100779493e4_f64 * t1735 * t10907 - 0.51947267698127589899e2_f64 * t4924 * t4958 + 0.1038945353962551798e3_f64 * t1735 * t10915 - 0.17544670192365612213e1_f64 * t10918 * t1747 - 0.17544670192365612213e1_f64 * t4924 * t4950 + 0.51725014705706168417e3_f64 * t10925 * t10929 + 0.19751789702565206229e-1_f64 * t45 * t10968 * t634 + 3.0_f64 * t10972 * t1726;
    t10975
}
