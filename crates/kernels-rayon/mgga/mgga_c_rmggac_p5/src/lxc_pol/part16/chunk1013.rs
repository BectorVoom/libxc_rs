//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1013/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1013(t38701: f64, t8417: f64, t46685: f64, t903: f64, t2310: f64, t9090: f64, t2283: f64, t10078: f64, t7244: f64, t1550: f64, t46369: f64, t10088: f64, t3351: f64, t498: f64, t511: f64, t9210: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47173 = t38701 * t8417;
    let t47175 = t903 * t46685;
    let t47178 = t9090 * t2310;
    let t47180 = t9090 * t2283;
    let t47182 = t7244 * t10078;
    let t47188 = t1550 * t46369;
    let t47196 = t3351 * t9210 * t511 * t10088 * t498;
    (t47173, t47175, t47178, t47180, t47182, t47188, t47196)
}
