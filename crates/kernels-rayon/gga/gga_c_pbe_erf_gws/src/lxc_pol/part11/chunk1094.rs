//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1094/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1094(t40039: f64, t40042: f64, t30583: f64, t30593: f64, t1044: f64, t1620: f64, t41690: f64, t7216: f64, t32019: f64, t3403: f64, t30660: f64, t40696: f64, t950: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47545 = 32.0_f64 / 15.0_f64 * t40039;
    let t47546 = 16.0_f64 / 45.0_f64 * t40042;
    let t47547 = 32.0_f64 / 135.0_f64 * t30583;
    let t47548 = 16.0_f64 / 45.0_f64 * t30593;
    let t47552 = 32.0_f64 / 5.0_f64 * t1620 * t7216 * t41690 * t1044;
    let t47554 = 32.0_f64 / 15.0_f64 * t32019 * t3403;
    let t47555 = 64.0_f64 / 135.0_f64 * t30660;
    let t47556 = t40696 * t950;
    (t47545, t47546, t47547, t47548, t47552, t47554, t47555, t47556)
}
