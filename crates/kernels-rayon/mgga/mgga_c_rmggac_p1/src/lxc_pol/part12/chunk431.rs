//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 431/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk431(t4248: f64, t4333: f64, t156: f64, t155: f64, t1132: f64, t385: f64, t1045: f64, t980: f64, t1034: f64, t2: f64, t428: f64, t180: f64, t243: f64, t483: f64) -> (f64, f64, f64, f64, f64) {
    let t4334 = t4248 + t4333;
    let t4335 = t156 * t4334;
    let t4336 = t155 * t4335;
    let t4338 = 12.0_f64 * t385 * t1132;
    let t4342 = t1045 * t980;
    let t4344 = t1034 * t2;
    let t4345 = t4344 * t428;
    let t4349 = t243 * t483 * t180;
    (t4336, t4338, t4342, t4345, t4349)
}
