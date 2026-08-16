//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1043/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1043(t16043: f64, t9975: f64, t1704: f64, t236: f64, t3351: f64, t35155: f64, t498: f64, t38701: f64, t8417: f64, t46685: f64, t903: f64, t2310: f64, t9090: f64) -> (f64, f64, f64, f64, f64) {
    let t47162 = t16043 * t9975;
    let t47167 = t3351 * t35155 * t236 * t1704 * t498;
    let t47173 = t38701 * t8417;
    let t47175 = t903 * t46685;
    let t47178 = t9090 * t2310;
    (t47162, t47167, t47173, t47175, t47178)
}
