//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 542/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk542(t437: f64, t7733: f64, t1537: f64, t1760: f64, t360: f64, t18: f64, t1577: f64) -> (f64, f64, f64, f64, f64) {
    let t7734 = t7733 * t437;
    let t7736 = t1537 * t1760;
    let t7741 = t360 * t360;
    let t7742 = 1.0_f64 / t7741;
    let t7743 = t18 * t7742;
    let t7745 = 6.0_f64 * t1577 - 6.0_f64 * t7743;
    (t7734, t7736, t7741, t7742, t7745)
}
