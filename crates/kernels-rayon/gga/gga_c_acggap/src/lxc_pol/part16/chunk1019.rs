//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1019/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1019(t30225: f64, t542: f64, t1588: f64, t7605: f64, t2327: f64, t7610: f64, t537: f64, t1576: f64, t1581: f64, t30811: f64, t4277: f64, t1466: f64, t30540: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35949 = t30225 * t542;
    let t35951 = t7605 * t1588;
    let t35952 = 0.17149607247227894789e-2_f64 * t35951;
    let t35955 = t7610 * t2327;
    let t35959 = t30225 * t537;
    let t35961 = t7605 * t1576;
    let t35962 = 0.17149607247227894789e-2_f64 * t35961;
    let t35963 = t7605 * t1581;
    let t35964 = 0.17149607247227894789e-2_f64 * t35963;
    let t35967 = t30811 * t4277;
    let t35968 = 0.68598428988911579156e-2_f64 * t35967;
    let t35969 = t30540 * t1466;
    (t35949, t35952, t35955, t35959, t35962, t35964, t35968, t35969)
}
