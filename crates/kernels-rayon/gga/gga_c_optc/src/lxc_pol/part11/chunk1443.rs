//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1443/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1443(t155: f64, t15776: f64, t15865: f64, t17662: f64, t18131: f64, t3103: f64, t3108: f64, t3234: f64, t36566: f64, t36641: f64, t4387: f64, t44001: f64, t4450: f64, t4457: f64, t45770: f64, t46242: f64, t46298: f64, t46314: f64, t5101: f64, t5329: f64, t5408: f64, t55555: f64, t55643: f64, t55645: f64, t60041: f64, t8974: f64, t9128: f64, t9175: f64) -> f64 {
    let t60199 = -0.5392791351917231181e5_f64 * t9175 * t5329 * t3108 * t55555 + 0.59919903910191457566e4_f64 * t9128 * t5329 * t155 * t15776 - 0.34343354033733364364e0_f64 * t36566 - 0.35826278725947873626e0_f64 * t55643 + 0.59710464543246456043e-1_f64 * t55645 - 0.779739765264702906e1_f64 * t46242 - 0.1559479530529405812e3_f64 * t3234 * t4387 * t60041 + 0.23442632909977165232e4_f64 * t4457 * t44001 * t8974 * t5101 + 0.59710464543246456046e-2_f64 * t36641 - 0.11195712101858710508e-1_f64 * t46298 - 0.30972456242994093474e2_f64 * t46314 - 0.12363607452144011171e1_f64 * t4450 * t18131 + 0.61944912485988186947e2_f64 * t3103 * t15865 * t17662 + 0.4707813348935102208e4_f64 * t45770 * t5408;
    t60199
}
