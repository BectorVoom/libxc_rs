//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1064/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1064(t25986: f64, t5609: f64, t2661: f64, t13846: f64, t1941: f64, t2018: f64, t5617: f64, t807: f64, t241: f64, t25981: f64, t820: f64, t25997: f64, t5665: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27928 = t25986 * t5609;
    let t27929 = t2661 * t27928;
    let t27932 = t1941 * t13846;
    let t27936 = t2018 * t5617;
    let t27937 = t807 * t27936;
    let t27940 = t820 * t25981 * t241;
    let t27953 = t25997 * t5665;
    (t27928, t27929, t27932, t27936, t27937, t27940, t27953)
}
