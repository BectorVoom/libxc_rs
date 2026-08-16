//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 499/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk499(t2880: f64, t458: f64, t2879: f64, t119: f64, t462: f64, t125: f64, t4: f64, t173: f64, t144: f64, t188: f64, t152: f64, t1947: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2881 = t2880 * t458;
    let t2882 = t2879 * t2881;
    let t2884 = t462 * t119;
    let t2885 = t4 * t125;
    let t2886 = t2885 * t173;
    let t2887 = t2884 * t2886;
    let t2889 = t188 * t144;
    let t2890 = t2889 * t152;
    let t2891 = t2890 * t1947;
    (t2881, t2882, t2884, t2885, t2886, t2887, t2890, t2891)
}
