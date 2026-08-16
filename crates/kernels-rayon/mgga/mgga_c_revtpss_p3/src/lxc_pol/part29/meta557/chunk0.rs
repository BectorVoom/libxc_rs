//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1899/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1899(t1444: f64, t5659: f64, t1955: f64, t25949: f64, t1883: f64, t4131: f64, t1903: f64, t3923: f64, t4003: f64, t2453: f64, t27883: f64, t27836: f64, t4075: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97839 = t5659 * t1444;
    let t97855 = t1955 * t25949;
    let t97858 = t1883 * t4131;
    let t97870 = t1903 * t3923;
    let t97871 = t97870 * t4003;
    let t97916 = t2453 * t27883;
    let t97933 = t1955 * t27836 * t4075;
    (t97839, t97855, t97858, t97870, t97871, t97916, t97933)
}
