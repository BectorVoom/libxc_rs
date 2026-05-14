//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 914/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk914<F: Float>(t30225: F, t537: F, t1576: F, t7605: F, t1581: F, t30811: F, t4277: F, t1466: F, t30540: F, t1470: F, t1549: F, t30644: F, t1554: F, t1558: F, t4326: F, t7647: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35959 = t30225 * t537;
    let t35961 = t7605 * t1576;
    let t35963 = t7605 * t1581;
    let t35967 = t30811 * t4277;
    let t35969 = t30540 * t1466;
    let t35973 = t30540 * t1470;
    let t35975 = t30644 * t1549;
    let t35977 = t30644 * t1554;
    let t35979 = t30644 * t1558;
    let t35981 = t7647 * t4326;
    (t35959, t35961, t35963, t35967, t35969, t35973, t35975, t35977, t35979, t35981)
}
