//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 871/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk871(t213: f64, t7910: f64, t5629: f64, t7271: f64, t1885: f64, t26024: f64, t25972: f64, t5622: f64, t1889: f64, t25978: f64, t25986: f64, t5609: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27909 = t213 * t7910;
    let t27919 = t7271 * t5629;
    let t27921 = t26024 * t1885;
    let t27924 = t25972 * t5622;
    let t27926 = t25978 * t1889;
    let t27928 = t25986 * t5609;
    (t27909, t27919, t27921, t27924, t27926, t27928)
}
