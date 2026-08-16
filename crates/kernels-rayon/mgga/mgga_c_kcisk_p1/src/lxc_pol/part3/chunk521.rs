//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 521/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk521(t3507: f64, t492: f64, t1506: f64, t1505: f64, t1512: f64, t1504: f64, t497: f64, t1414: f64, t381: f64, t79: f64, t3742: f64, t3784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4223 = t3507 * t492;
    let t4224 = t4223 * t1506;
    let t4226 = t1512 * t1505;
    let t4227 = t1504 * t4226;
    let t4229 = t492 * t497;
    let t4230 = t1414 * t4229;
    let t4231 = t79 * t381;
    let t4232 = t4231 * t3742;
    let t4233 = t4230 * t4232;
    let t4235 = t3784 * t492;
    (t4223, t4224, t4226, t4227, t4229, t4230, t4231, t4232, t4233, t4235)
}
