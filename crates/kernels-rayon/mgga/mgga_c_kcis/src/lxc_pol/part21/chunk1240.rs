//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1240/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1240(t1183: f64, t982: f64, t7771: f64, t92794: f64, t1014: f64, t26811: f64, t26987: f64, t7784: f64, t26960: f64, t92850: f64, t26833: f64, t3245: f64, t7723: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93059 = t1183 * t982;
    let t93082 = t7771 * t92794;
    let t93087 = t1014 * t26811;
    let t93099 = t26987 * t7784;
    let t93134 = t26960 * t92850;
    let t93143 = t1014 * t26833;
    let t93145 = t3245 * t7723;
    (t93059, t93082, t93087, t93099, t93134, t93143, t93145)
}
