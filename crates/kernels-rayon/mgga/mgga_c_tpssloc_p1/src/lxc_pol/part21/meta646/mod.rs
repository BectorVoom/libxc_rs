//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta646 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2439;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2440;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta646(t3147: f64, t698: f64, t973: f64, t10632: f64, t2924: f64, t10510: f64, t3114: f64, t10508: f64, t248: f64, t3039: f64, t3041: f64, t3020: f64, t3030: f64, t3032: f64, t3038: f64, t1043: f64, t204: f64, t1041: f64, t884: f64, t10189: f64, t3014: f64, t10337: f64, t964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42613, t42671, t42721, t42735, t42741) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2439(t3147, t698, t973, t10632, t2924, t10510, t3114, t10508, t248, t3039, t3041, t3020, t3030);
        let (t42742, t42743, t42749, t42752, t42771, t42811) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2440(t3032, t42741, t3038, t1043, t204, t1041, t248, t884, t10189, t3014, t10337, t964);
    (t42613, t42671, t42721, t42735, t42741, t42742, t42743, t42749, t42752, t42771, t42811)
}
