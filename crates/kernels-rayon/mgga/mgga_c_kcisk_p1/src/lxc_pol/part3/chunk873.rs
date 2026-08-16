//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 873/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk873(t13121: f64, t321: f64, t1201: f64, t13050: f64, t13053: f64, t13056: f64, t13060: f64, t13066: f64, t13101: f64, t13105: f64, t3692: f64, t3699: f64, t3718: f64, t3726: f64) -> (f64, f64) {
    let t13123 = 0.62182e-1_f64 * t13121 * t321;
    let t13124 = -0.17544670192365612213e1_f64 * t3692 * t3718 - t13050 + t13053 - t13056 + t13060 - 0.51947267698127589899e2_f64 * t3692 * t3726 + 0.1038945353962551798e3_f64 * t1201 * t13066 - 0.58482233974552040708e0_f64 * t1201 * t13101 - 0.35089340384731224426e1_f64 * t1201 * t13105 + 0.35089340384731224426e1_f64 * t3692 * t3699 - t13123;
    (t13123, t13124)
}
