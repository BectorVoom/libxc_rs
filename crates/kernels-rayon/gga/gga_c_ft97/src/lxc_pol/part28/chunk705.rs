//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 705/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk705(t27063: f64, t27112: f64, t27141: f64, t27189: f64, t605: f64, t144: f64, t11593: f64, t1901: f64, t26978: f64, t26982: f64, t26985: f64, t26988: f64, t26992: f64, t26996: f64, t27001: f64, t27004: f64, t27008: f64, t27012: f64, t27017: f64, t27022: f64, t27025: f64, t446: f64) -> (f64, f64) {
    let t27191 = t27063 + t27112 + t27141 + t27189;
    let t27192 = t605 * t27191;
    let t27193 = t144 * t27192;
    let t27196 = -t446 * t26978 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t26982 + t1901 * t26985 / 9.0_f64 + t1901 * t26988 / 9.0_f64 + t1901 * t26992 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t11593 * t26996 - 2.0_f64 * t1901 * t27001 + t27004 / 9.0_f64 + t1901 * t27008 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t27012 - 2.0_f64 / 3.0_f64 * t1901 * t27017 + t1901 * t27022 / 9.0_f64 - t27025 / 27.0_f64 - t446 * t27193 / 3.0_f64;
    (t27191, t27196)
}
