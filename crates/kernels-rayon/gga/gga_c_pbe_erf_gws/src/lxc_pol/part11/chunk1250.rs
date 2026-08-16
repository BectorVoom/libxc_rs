//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1250/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1250(t44650: f64, t9016: f64, t1105: f64, t3854: f64, t8884: f64, t3138: f64, t4386: f64, t3824: f64, t20933: f64, t21298: f64, t858: f64, t867: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49839 = t9016 * t44650 / 8.0_f64;
    let t49841 = t1105 * t3854;
    let t49842 = t8884 * t49841;
    let t49845 = t3138 * t4386 * t49842 / 2.0_f64;
    let t49847 = t3824 * t3824;
    let t49848 = t49847 * t20933;
    let t49852 = t21298 * t867 * t858 * t49848 / 4.0_f64;
    (t49839, t49841, t49842, t49845, t49847, t49848, t49852)
}
