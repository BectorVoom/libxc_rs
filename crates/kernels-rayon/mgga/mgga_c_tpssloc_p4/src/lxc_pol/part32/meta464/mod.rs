//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1751;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1752;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta464(t2109: f64, t22550: f64, t7245: f64, t9231: f64, t33: f64, t7254: f64, t2240: f64, t1235: f64, t7299: f64, t2127: f64, t23383: f64, t7303: f64, t7291: f64, t2123: f64, t3427: f64, t2121: f64, t221: f64, t3448: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24517, t24520, t24525, t24526, t24567, t24574) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1751(t2109, t22550, t7245, t9231, t33, t7254, t2240, t1235, t7299, t2127, t23383);
        let (t24575, t24577, t24585, t24587, t24588, t24589) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1752(t24574, t7303, t7291, t2123, t3427, t2121, t221, t3448, t2127);
    (t24517, t24520, t24525, t24526, t24567, t24574, t24575, t24577, t24585, t24587, t24588, t24589)
}
