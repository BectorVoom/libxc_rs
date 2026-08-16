//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 457/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk457(t429: f64, t529: f64, t496: f64, t492: f64, t490: f64, t149: f64, t209: f64, t371: f64, t1135: f64, t56: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2837 = t529 * t429;
    let t2838 = t2837 * t496;
    let t2839 = t492 * t2838;
    let t2841 = t490 * t2839 / 9.0_f64;
    let t2843 = t209 * t149 * t371;
    let t2844 = 0.25851111111111111111e1_f64 * t2843;
    let t2847 = t56 * t1135;
    (t2837, t2838, t2839, t2841, t2843, t2844, t2847)
}
