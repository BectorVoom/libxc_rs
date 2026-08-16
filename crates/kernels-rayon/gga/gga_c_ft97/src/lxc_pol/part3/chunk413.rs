//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 413/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk413(t5: f64, t885: f64, t170: f64, t2248: f64, t328: f64, t2253: f64, t895: f64, t906: f64, t70: f64, t703: f64, t2347: f64, t327: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2904 = t5 * t885;
    let t2912 = 5.0_f64 / 18.0_f64 * t170 * t2248 * t328;
    let t2913 = t2253 * t895;
    let t2915 = t2253 * t906;
    let t2917 = t70 * t703;
    let t2918 = t327 * t2347;
    (t2904, t2912, t2913, t2915, t2917, t2918)
}
