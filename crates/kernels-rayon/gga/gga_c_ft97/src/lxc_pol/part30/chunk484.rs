//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 484/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk484(t7611: f64, t799: f64, t27: f64, t89: f64, t7644: f64, t7648: f64, t7652: f64, t7656: f64) -> (f64, f64, f64) {
    let t7658 = t799 * t7611;
    let t7660 = t89 * t27 * t7658;
    let t7662 = -t7644 / 3.0_f64 + t7648 / 3.0_f64 - t7652 / 6.0_f64 + 2.0_f64 / 3.0_f64 * t7656 - t7660 / 3.0_f64;
    (t7658, t7660, t7662)
}
