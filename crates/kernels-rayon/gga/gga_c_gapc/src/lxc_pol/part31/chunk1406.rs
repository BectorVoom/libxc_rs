//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1406/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1406(t35137: f64, t35141: f64, t35143: f64, t35146: f64, t35152: f64, t35155: f64, t35157: f64, t35160: f64, t35162: f64, t35169: f64, t35173: f64, t35184: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37211 = 0.3243554543208642639e-2_f64 * t35137;
    let t37212 = 0.61551119569641057312e-8_f64 * t35141;
    let t37213 = 0.27012148473991046866e-5_f64 * t35143;
    let t37214 = 0.11372686522837130914e-5_f64 * t35146;
    let t37216 = 0.337303223138432284e-8_f64 * t35152;
    let t37217 = 0.55331893559454114829e-8_f64 * t35155;
    let t37218 = 0.99044544404633838508e-5_f64 * t35157;
    let t37219 = 0.33816362383187442026e-5_f64 * t35160;
    let t37220 = 0.80192315782160920384e-6_f64 * t35162;
    let t37223 = 0.11984097313886885523e-6_f64 * t35169;
    let t37224 = 0.63350674672043801542e-5_f64 * t35173;
    let t37227 = 0.69504740211613770836e-3_f64 * t35184;
    (t37211, t37212, t37213, t37214, t37216, t37217, t37218, t37219, t37220, t37223, t37224, t37227)
}
