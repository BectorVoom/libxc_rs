//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 288/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk288(t18: f64, t738: f64, t737: f64, t1152: f64, t458: f64, t2493: f64, t3713: f64, t1131: f64, t2: f64, t2372: f64, t713: f64, t192: f64, t3821: f64, t743: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3921 = t738 * t18;
    let t3922 = t737 * t3921;
    let t3925 = t458 * t1152;
    let t3927 = t2493 * t3713;
    let t3930 = t2 * t1131;
    let t3932 = t2372 * t3930 * t713;
    let t3936 = t192 * t743 * t3821;
    (t3921, t3922, t3925, t3927, t3932, t3936)
}
