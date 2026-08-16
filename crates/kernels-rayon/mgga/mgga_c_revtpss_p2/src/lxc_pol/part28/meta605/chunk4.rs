//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2094/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2094(t1955: f64, t25949: f64, t1883: f64, t4131: f64, t1904: f64, t25912: f64, t689: f64, t1903: f64, t3923: f64, t4003: f64, t1385: f64, t7910: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97855 = t1955 * t25949;
    let t97858 = t1883 * t4131;
    let t97869 = 0.10975748638225852664e-1_f64 * t689 * t25912 * t1904;
    let t97870 = t1903 * t3923;
    let t97871 = t97870 * t4003;
    let t97875 = t1385 * t7910;
    (t97855, t97858, t97869, t97870, t97871, t97875)
}
