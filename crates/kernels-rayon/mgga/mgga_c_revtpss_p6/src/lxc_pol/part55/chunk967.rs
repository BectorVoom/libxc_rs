//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 967/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk967(t1310: f64, t7983: f64, t7315: f64, t8108: f64, t13648: f64, t2107: f64, t28683: f64, t508: f64, t22496: f64, t26405: f64, t5542: f64, t7536: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28704 = t1310 * t7983;
    let t28707 = t8108 * t7315;
    let t28709 = t2107 * t13648;
    let t28711 = t508 * t28683;
    let t28718 = t26405 * t22496;
    let t28727 = t7536 * t5542;
    (t28704, t28707, t28709, t28711, t28718, t28727)
}
