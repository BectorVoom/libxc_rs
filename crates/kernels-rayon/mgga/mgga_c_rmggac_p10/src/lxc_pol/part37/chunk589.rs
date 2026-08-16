//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 589/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk589(t15163: f64, t7788: f64, t15090: f64, t262: f64, t7782: f64, t15084: f64, t7835: f64, t15078: f64, t793: f64, t15049: f64, t305: f64, t15075: f64, t3851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15164 = t7788 * t15163;
    let t15166 = t262 * t15090;
    let t15167 = t7782 * t15166;
    let t15169 = t262 * t15084;
    let t15170 = t7835 * t15169;
    let t15172 = t793 * t15078;
    let t15175 = 0.2993560425465952141e-1_f64 * t305 * t15049;
    let t15176 = t3851 * t15075;
    (t15164, t15166, t15167, t15169, t15170, t15172, t15175, t15176)
}
