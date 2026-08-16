//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 551/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk551(t6150: f64, t681: f64, t89: f64, t1882: f64, t6168: f64, t24482: f64, t24537: f64, t1445: f64, t2399: f64, t1449: f64, t2567: f64, t6163: f64, t8392: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24605 = t89 * t681 * t6150;
    let t24611 = t1882 * t6168;
    let t24628 = 4.0_f64 / 27.0_f64 * t24482;
    let t24642 = 2.0_f64 / 27.0_f64 * t24537;
    let t24658 = 4.0_f64 / 27.0_f64 * t89 * t2399 * t1445;
    let t24668 = t2567 * t1449;
    let t24673 = t8392 * t6163;
    (t24605, t24611, t24628, t24642, t24658, t24668, t24673)
}
