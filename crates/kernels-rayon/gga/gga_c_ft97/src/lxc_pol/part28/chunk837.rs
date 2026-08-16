//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 837/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk837(t1800: f64, t34415: f64, t1317: f64, t28: f64, t7172: f64, t939: f64, t32252: f64, t32253: f64, t930: f64, t6437: f64, t7853: f64, t32147: f64, t6441: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34416 = t1800 * t34415;
    let t34418 = t1317 * t28 * t34416;
    let t34421 = t7172 * t939;
    let t34424 = t32252 * t32253 * t930;
    let t34427 = t7853 * t6437;
    let t34430 = t32147 * t6441;
    (t34416, t34418, t34421, t34424, t34427, t34430)
}
