//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1443/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1443<F: Float>(t155: F, t15776: F, t15865: F, t17662: F, t18131: F, t3103: F, t3108: F, t3234: F, t36566: F, t36641: F, t4387: F, t44001: F, t4450: F, t4457: F, t45770: F, t46242: F, t46298: F, t46314: F, t5101: F, t5329: F, t5408: F, t55555: F, t55643: F, t55645: F, t60041: F, t8974: F, t9128: F, t9175: F) -> F {
    let t60199 = -F::new(0.5392791351917231181e5) * t9175 * t5329 * t3108 * t55555 + F::new(0.59919903910191457566e4) * t9128 * t5329 * t155 * t15776 - F::new(0.34343354033733364364e0) * t36566 - F::new(0.35826278725947873626e0) * t55643 + F::new(0.59710464543246456043e-1) * t55645 - F::new(0.779739765264702906e1) * t46242 - F::new(0.1559479530529405812e3) * t3234 * t4387 * t60041 + F::new(0.23442632909977165232e4) * t4457 * t44001 * t8974 * t5101 + F::new(0.59710464543246456046e-2) * t36641 - F::new(0.11195712101858710508e-1) * t46298 - F::new(0.30972456242994093474e2) * t46314 - F::new(0.12363607452144011171e1) * t4450 * t18131 + F::new(0.61944912485988186947e2) * t3103 * t15865 * t17662 + F::new(0.4707813348935102208e4) * t45770 * t5408;
    t60199
}
