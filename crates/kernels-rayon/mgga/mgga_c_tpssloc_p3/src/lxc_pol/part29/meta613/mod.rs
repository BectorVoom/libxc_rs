//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2053;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2054;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta613(t24591: f64, t85639: f64, t24698: f64, t491: f64, t2127: f64, t82631: f64, t7291: f64, t24564: f64, t24574: f64, t11605: f64, t225: f64, t3597: f64, t3599: f64, t2122: f64, t7303: f64, t3590: f64, t7299: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t85640, t85648, t85660) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2053(t24591, t85639, t24698, t491, t2127, t82631);
        let (t85661, t85669, t85674, t85688, t85701, t85707) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2054(t7291, t85660, t24564, t24574, t11605, t225, t3597, t3599, t2122, t7303, t3590, t7299);
    (t85640, t85648, t85660, t85661, t85669, t85674, t85688, t85701, t85707)
}
