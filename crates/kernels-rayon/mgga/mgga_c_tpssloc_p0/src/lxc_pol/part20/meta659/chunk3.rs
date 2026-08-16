//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2455/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2455(t50262: f64, t10875: f64, t48569: f64, t10879: f64, t10904: f64, t13977: f64, t13987: f64, t14001: f64, t14006: f64, t2960: f64, t42561: f64, t43228: f64, t43233: f64, t47701: f64, t50242: f64, t50250: f64, t50255: f64, t50259: f64, t973: f64, t977: f64) -> f64 {
    let t50263 = t50262 / 6912.0_f64;
    let t50265 = t48569 * t10875;
    let t50268 = t43228 / 432.0_f64 + t2960 * t14001 / 9.0_f64 - t50242 / 72.0_f64 + t973 * t977 * t47701 / 16.0_f64 + t2960 * t14006 / 18.0_f64 - t50250 / 144.0_f64 - t42561 * t13987 / 32.0_f64 + t50255 / 256.0_f64 + t50259 - t10904 * t13977 / 48.0_f64 - t50263 - t43233 / 1536.0_f64 - t50265 * t10879 / 512.0_f64;
    t50268
}
