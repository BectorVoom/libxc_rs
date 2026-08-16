//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 790/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk790(t4831: f64, t973: f64, t4851: f64, t993: f64, t4854: f64, t7341: f64, t2367: f64, t5068: f64, t999: f64, t7501: f64, t2418: f64, t4814: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13733 = t4831 * t973;
    let t13794 = t4851 * t993;
    let t13796 = t7341 * t4854;
    let t13802 = t2367 * t5068;
    let t13803 = t999 * t13802;
    let t13842 = t7501 * t4854;
    let t13890 = t4814 * t2418;
    (t13733, t13794, t13796, t13802, t13803, t13842, t13890)
}
