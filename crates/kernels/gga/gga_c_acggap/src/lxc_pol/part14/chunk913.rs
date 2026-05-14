//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 913/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk913<F: Float>(t1588: F, t7605: F, t2327: F, t7610: F, t30225: F, t537: F, t1576: F, t1581: F, t30811: F, t4277: F, t1466: F, t30540: F, t1470: F, t1549: F, t30644: F, t1554: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35951 = t7605 * t1588;
    let t35952 = 0.17149607247227894789e-2 * t35951;
    let t35955 = t7610 * t2327;
    let t35959 = t30225 * t537;
    let t35961 = t7605 * t1576;
    let t35962 = 0.17149607247227894789e-2 * t35961;
    let t35963 = t7605 * t1581;
    let t35964 = 0.17149607247227894789e-2 * t35963;
    let t35967 = t30811 * t4277;
    let t35968 = 0.68598428988911579156e-2 * t35967;
    let t35969 = t30540 * t1466;
    let t35973 = t30540 * t1470;
    let t35975 = t30644 * t1549;
    let t35976 = 0.17149607247227894789e-2 * t35975;
    let t35977 = t30644 * t1554;
    (t35952, t35955, t35959, t35962, t35964, t35968, t35969, t35973, t35976, t35977)
}
