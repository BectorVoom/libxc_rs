//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 935/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk935(t3866: f64, t6431: f64, t120: f64, t6414: f64, t225: f64, t6364: f64, t6435: f64, t6362: f64, t1390: f64, t6463: f64, t3701: f64, t6324: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19942 = t3866 * t6431;
    let t19956 = t120 * t6414;
    let t20029 = t6364 * t225;
    let t20044 = t6435 * t225;
    let t20060 = t6362 * t225;
    let t20067 = t6463 * t1390;
    let t20077 = t6324 * t3701;
    (t19942, t19956, t20029, t20044, t20060, t20067, t20077)
}
