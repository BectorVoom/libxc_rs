//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1686/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1686(t10: f64, t22: f64, t576: f64, t588: f64, t15: f64, t27: f64, t11: f64, t10276: f64, t2224: f64, t584: f64, t596: f64, t20: f64, t2237: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45926 = t10 * t22;
    let t45927 = 72.0_f64 * t45926;
    let t45928 = t576 * t588;
    let t45929 = 192.0_f64 * t45928;
    let t45931 = 120.0_f64 * t15 * t27;
    let t45933 = 24.0_f64 * t11 * t22;
    let t45934 = t10276 * t588;
    let t45935 = 384.0_f64 * t45934;
    let t45936 = t2224 * t27;
    let t45937 = 1440.0_f64 * t45936;
    let t45938 = t584 * t596;
    let t45939 = 1920.0_f64 * t45938;
    let t45941 = 840.0_f64 * t20 * t2237;
    (t45927, t45929, t45931, t45933, t45935, t45937, t45939, t45941)
}
