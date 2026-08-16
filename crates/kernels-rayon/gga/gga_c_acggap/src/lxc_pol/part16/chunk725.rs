//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 725/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk725(t601: f64, t7780: f64, t606: f64, t1973: f64, t1988: f64, t1982: f64, t1983: f64, t361: f64, t1980: f64, t1979: f64, t377: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7781 = t7780 * t601;
    let t7782 = 0.45017719023973223821e-2_f64 * t7781;
    let t7787 = t7780 * t606;
    let t7788 = 0.66040993808168719343e-2_f64 * t7787;
    let t7789 = t1988 * t1973;
    let t7796 = t1982 * t361 * t1983;
    let t7797 = t1980 * t7796;
    let t7799 = t377 * t1979;
    (t7782, t7788, t7789, t7796, t7797, t7799)
}
