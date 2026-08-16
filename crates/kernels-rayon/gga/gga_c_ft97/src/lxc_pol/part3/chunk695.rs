//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 695/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk695(t10992: f64, t11021: f64, t11023: f64, t11025: f64, t11069: f64, t11416: f64, t3206: f64, t8392: f64, t100: f64, t8275: f64, t103: f64, t7763: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11922 = 2.0_f64 / 9.0_f64 * t10992;
    let t11930 = 2.0_f64 / 9.0_f64 * t11021;
    let t11931 = 4.0_f64 / 9.0_f64 * t11023;
    let t11932 = 4.0_f64 / 27.0_f64 * t11025;
    let t11946 = 2.0_f64 / 3.0_f64 * t11069;
    let t11957 = 4.0_f64 / 3.0_f64 * t11416;
    let t11981 = 2.0_f64 / 27.0_f64 * t8392 * t3206;
    let t11987 = t8275 * t100;
    let t11988 = t103 * t7763;
    (t11922, t11930, t11931, t11932, t11946, t11957, t11981, t11987, t11988)
}
