//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 776/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk776(t3829: f64, t543: f64, t3937: f64, t9955: f64, t1386: f64, t820: f64, t844: f64, t3940: f64, t1371: f64, t3889: f64, t800: f64, t221: f64, t3924: f64, t4019: f64) -> (f64, f64, f64, f64) {
    let t9956 = t543 * t3829;
    let t9958 = t9955 * t3937 * t9956;
    let t9962 = t820 * t1386 * t844;
    let t9963 = t9962 * t3940;
    let t9966 = t800 * t1371 * t3889;
    let t9970 = t4019 * t221 * t3924;
    (t9958, t9963, t9966, t9970)
}
