//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2128;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2129;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta568(t3147: f64, t698: f64, t973: f64, t10981: f64, t2960: f64, t10984: f64, t1004: f64, t10956: f64, t10863: f64, t3053: f64, t10516: f64, t3113: f64, t1012: f64, t1015: f64, t1017: f64, t10444: f64, t10632: f64, t2924: f64, t10510: f64, t3114: f64, t10454: f64, t3117: f64, t10891: f64, t10895: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42613, t42619, t42622, t42648, t42651, t42653) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2128(t3147, t698, t973, t10981, t2960, t10984, t1004, t10956, t10863, t3053, t10516, t3113);
        let (t42658, t42671, t42721, t42729, t42731) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2129(t1012, t1015, t1017, t10444, t10632, t2924, t10510, t3114, t10454, t3117, t10891, t10895);
    (t42613, t42619, t42622, t42648, t42651, t42653, t42658, t42671, t42721, t42729, t42731)
}
