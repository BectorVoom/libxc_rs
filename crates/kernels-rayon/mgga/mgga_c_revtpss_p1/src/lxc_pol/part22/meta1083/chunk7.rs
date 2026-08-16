//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3920/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3920(t1448: f64, t5778: f64, t13625: f64, t13674: f64, t21937: f64, t22483: f64, t3889: f64, t4139: f64, t47084: f64, t49582: f64, t5541: f64, t5542: f64, t74114: f64, t74115: f64, t74116: f64, t74117: f64, t74119: f64, t74120: f64) -> f64 {
    let t75365 = t5778 * t1448;
    let t75372 = -6.0_f64 * t13625 * t22483 * t4139 + 8.0_f64 * t13674 * t5541 * t75365 + 3.0_f64 * t21937 * t3889 * t4139 - 6.0_f64 * t4139 * t49582 * t5542 - t47084 - t74114 + t74115 + t74116 - t74117 + t74119 - t74120;
    t75372
}
