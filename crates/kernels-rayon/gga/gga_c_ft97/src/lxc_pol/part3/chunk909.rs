//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 909/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk909(t16579: f64, t704: f64, t420: f64, t701: f64, t17766: f64, t3806: f64, t13605: f64, t17749: f64, t13609: f64, t17753: f64, t17744: f64, t2320: f64) -> (f64, f64, f64, f64, f64) {
    let t18049 = t704 * t16579;
    let t18050 = t420 * t18049;
    let t18051 = t701 * t18050;
    let t18054 = t3806 * t17766;
    let t18055 = t701 * t18054;
    let t18057 = t13605 * t17749;
    let t18058 = t701 * t18057;
    let t18060 = t13609 * t17753;
    let t18061 = t701 * t18060;
    let t18063 = t2320 * t17744;
    (t18051, t18055, t18058, t18061, t18063)
}
