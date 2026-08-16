//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 822/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk822(t193: f64, t35269: f64, t1454: f64, t6838: f64, t1173: f64, t7485: f64, t1131: f64, t6008: f64, t7441: f64, t6154: f64, t6940: f64, t1449: f64, t28023: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35270 = t193 * t35269;
    let t35275 = t6838 * t1454;
    let t35276 = t193 * t35275;
    let t35281 = t7485 * t1173;
    let t35282 = t193 * t35281;
    let t35285 = t1454 * t1131;
    let t35286 = t6008 * t35285;
    let t35287 = t193 * t35286;
    let t35296 = t7441 * t1173;
    let t35297 = t193 * t35296;
    let t35302 = t6154 * t6940;
    let t35304 = t28023 * t1449;
    (t35270, t35275, t35276, t35281, t35282, t35285, t35286, t35287, t35296, t35297, t35302, t35304)
}
