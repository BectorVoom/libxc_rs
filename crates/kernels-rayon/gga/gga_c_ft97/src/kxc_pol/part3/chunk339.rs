//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 339/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk339(t1882: f64, t449: f64, t104: f64, t1637: f64, t89: f64, t454: f64, t494: f64, t27: f64, t444: f64, t443: f64) -> (f64, f64, f64, f64, f64) {
    let t1883 = t1882 * t449;
    let t1887 = 4.0_f64 / 27.0_f64 * t89 * t1637 * t104;
    let t1888 = t1882 * t454;
    let t1890 = t1882 * t494;
    let t1900 = t444 * t27;
    let t1901 = t443 * t1900;
    (t1883, t1887, t1888, t1890, t1901)
}
