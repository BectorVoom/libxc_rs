//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1063/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1063(t1955: f64, t7283: f64, t1426: f64, t27836: f64, t7063: f64, t786: f64, t1885: f64, t26024: f64, t25972: f64, t5622: f64, t1889: f64, t25978: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27868 = t1955 * t7283;
    let t27883 = t27836 * t1426;
    let t27884 = t7063 * t27883;
    let t27899 = t786 * t27883;
    let t27921 = t26024 * t1885;
    let t27924 = t25972 * t5622;
    let t27926 = t25978 * t1889;
    (t27868, t27883, t27884, t27899, t27921, t27924, t27926)
}
