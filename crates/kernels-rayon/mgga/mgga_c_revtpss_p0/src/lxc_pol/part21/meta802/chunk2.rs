//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2916/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2916(t11105: f64, t11108: f64, t1699: f64, t3333: f64, t41937: f64, t5019: f64, t5023: f64, t52502: f64, t52507: f64, t52510: f64, t52885: f64, t52887: f64, t52889: f64, t52897: f64, t52899: f64, t52905: f64) -> f64 {
    let t52906 = -6.0_f64 * t11105 * t1699 * t41937 * t5023 + 6.0_f64 * t11108 * t3333 * t5019 * t5023 + t52502 - t52507 - t52510 + t52885 - t52887 - t52889 - t52897 + t52899 - t52905;
    t52906
}
