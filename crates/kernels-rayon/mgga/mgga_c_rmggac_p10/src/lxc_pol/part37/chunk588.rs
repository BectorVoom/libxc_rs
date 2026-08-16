//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 588/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk588(t14296: f64, t15137: f64, t14302: f64, t15106: f64, t14305: f64, t15109: f64, t3046: f64, t570: f64, t1326: f64, t14309: f64, t15087: f64, t262: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15138 = t14296 * t15137;
    let t15140 = t14302 * t15106;
    let t15142 = t14305 * t15109;
    let t15144 = t3046 * t570;
    let t15146 = t14309 * t1326 * t15144;
    let t15163 = t262 * t15087;
    (t15138, t15140, t15142, t15144, t15146, t15163)
}
