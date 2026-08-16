//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 320/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk320(t262: f64, t3080: f64, t2079: f64, t3046: f64, t793: f64, t637: f64, t797: f64, t1322: f64, t838: f64, t1326: f64) -> (f64, f64, f64, f64, f64) {
    let t3081 = t262 * t3080;
    let t3082 = t2079 * t3081;
    let t3088 = t793 * t3046;
    let t3091 = t797 * t3046 * t637;
    let t3093 = t838 * t1322;
    let t3094 = t1326 * t3046;
    (t3082, t3088, t3091, t3093, t3094)
}
