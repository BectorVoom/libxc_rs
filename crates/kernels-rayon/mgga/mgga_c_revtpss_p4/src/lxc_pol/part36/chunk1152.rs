//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1152/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1152(t25972: f64, t5622: f64, t1889: f64, t25978: f64, t25986: f64, t5609: f64, t2661: f64, t13846: f64, t1941: f64, t2018: f64, t5617: f64, t807: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27924 = t25972 * t5622;
    let t27926 = t25978 * t1889;
    let t27928 = t25986 * t5609;
    let t27929 = t2661 * t27928;
    let t27932 = t1941 * t13846;
    let t27936 = t2018 * t5617;
    let t27937 = t807 * t27936;
    (t27924, t27926, t27928, t27929, t27932, t27936, t27937)
}
