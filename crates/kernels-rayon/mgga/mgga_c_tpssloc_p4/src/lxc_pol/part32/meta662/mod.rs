//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2093;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta662(t7303: f64, t94490: f64, t7291: f64, t2122: f64, t94319: f64, t8034: f64, t8003: f64, t85660: f64, t24574: f64, t27412: f64, t5052: f64, t7299: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t94492, t94494, t94503, t94514, t94525, t94535, t94558) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2093(t7303, t94490, t7291, t2122, t94319, t8034, t8003, t85660, t24574, t27412, t5052, t7299);
    (t94492, t94494, t94503, t94514, t94525, t94535, t94558)
}
