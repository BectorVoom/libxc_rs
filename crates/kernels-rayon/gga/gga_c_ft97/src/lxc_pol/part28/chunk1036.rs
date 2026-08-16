//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1036/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1036(t3103: f64, t7211: f64, t1317: f64, t1800: f64, t28: f64, t34482: f64, t469: f64, t473: f64, t5665: f64, t32057: f64, t32063: f64, t34371: f64) -> (f64, f64, f64, f64) {
    let t145035 = t7211 * t3103;
    let t145038 = t1317 * t28 * t1800 * t145035;
    let t145042 = t5665 * t28 * t469 * t34482 * t473;
    let t145045 = t32057 * t32063 * t34371;
    (t145035, t145038, t145042, t145045)
}
