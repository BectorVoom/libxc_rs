//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 945/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk945(t2157: f64, t9428: f64, t2179: f64, t609: f64, t9258: f64, t159: f64, t9437: f64, t157: f64, t2180: f64, t9439: f64, t7806: f64, t9348: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39641 = t9428 * t2157;
    let t39646 = t2179 * t609 * t9258;
    let t39648 = t2157 * t2157;
    let t39649 = t2179 * t39648;
    let t39652 = 1.0_f64 / t9437 / t159;
    let t39653 = t157 * t39652;
    let t39654 = t2180 * t2180;
    let t39655 = t39653 * t39654;
    let t39658 = t9439 * t2180 * t2157;
    let t39660 = t7806 * t9348;
    (t39641, t39646, t39649, t39655, t39658, t39660)
}
