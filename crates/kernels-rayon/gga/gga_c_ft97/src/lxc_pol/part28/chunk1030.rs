//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1030/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1030(t1317: f64, t144958: f64, t1800: f64, t28: f64, t136138: f64, t144846: f64, t32067: f64, t144666: f64, t432: f64, t89: f64, t3103: f64, t32355: f64) -> (f64, f64, f64, f64) {
    let t144961 = t1317 * t28 * t1800 * t144958;
    let t144966 = t32067 * t136138 * t144846;
    let t144970 = t89 * t28 * t144666 * t432;
    let t144974 = t89 * t28 * t32355 * t3103;
    (t144961, t144966, t144970, t144974)
}
