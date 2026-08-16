//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1165/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1165(t30225: f64, t537: f64, t1576: f64, t7605: f64, t1581: f64, t2001: f64, t4849: f64, t30811: f64, t4277: f64, t1466: f64, t30540: f64, t4406: f64, t7822: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35959 = t30225 * t537;
    let t35961 = t7605 * t1576;
    let t35962 = 0.17149607247227894789e-2_f64 * t35961;
    let t35963 = t7605 * t1581;
    let t35964 = 0.17149607247227894789e-2_f64 * t35963;
    let t35965 = t2001 * t4849;
    let t35967 = t30811 * t4277;
    let t35968 = 0.68598428988911579156e-2_f64 * t35967;
    let t35969 = t30540 * t1466;
    let t35971 = t7822 * t4406;
    (t35959, t35962, t35964, t35965, t35968, t35969, t35971)
}
