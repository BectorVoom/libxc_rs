//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 722/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk722(t1286: f64, t31995: f64, t376: f64, t7213: f64, t497: f64, t7166: f64, t28: f64, t108: f64, t7211: f64, t379: f64, t1564: f64, t7161: f64, t92: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31997 = t1286 * t31995 / 9.0_f64;
    let t31998 = t376 * t7213;
    let t32000 = t1286 * t31998 / 18.0_f64;
    let t32001 = t7166 * t497;
    let t32002 = t28 * t32001;
    let t32011 = t7211 * t108;
    let t32012 = t32011 * t379;
    let t32013 = t1564 * t32012;
    let t32016 = t7161 * t92;
    (t31997, t31998, t32000, t32001, t32002, t32011, t32013, t32016)
}
