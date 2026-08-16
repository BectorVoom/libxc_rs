//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 464/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk464(t352: f64, t5098: f64, t4616: f64, t570: f64, t876: f64, t1357: f64, t866: f64, t1652: f64, t874: f64, t1615: f64, t333: f64, t1614: f64, t338: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5099 = t5098 * t352;
    let t5102 = t4616 * t570;
    let t5103 = t5102 * t876;
    let t5108 = t1357 * t866;
    let t5115 = t874 * t1652;
    let t5116 = t5115 * t352;
    let t5121 = t1615 * t333;
    let t5126 = t338 * t1614;
    (t5099, t5103, t5108, t5116, t5121, t5126)
}
