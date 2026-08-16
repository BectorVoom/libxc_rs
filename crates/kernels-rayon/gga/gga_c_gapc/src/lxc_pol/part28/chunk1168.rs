//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1168/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1168(t33657: f64, t786: f64, t3327: f64, t33655: f64, t7451: f64, t15507: f64, t22: f64, t5: f64, t18679: f64, t2763: f64, t3699: f64, t7730: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t33658 = t33657 * t786;
    let t33660 = t7451 * t33655 * t3327 * t33658;
    let t33666 = 1.0_f64 / t22 / t15507 * pi * t5;
    let t33670 = t3699 * t18679 * t2763 * t7730;
    (t33658, t33660, t33666, t33670)
}
