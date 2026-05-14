//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1035/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1035<F: Float>(t13426: F, t8460: F, t18227: F, t27123: F, t28219: F, t28019: F, t4147: F, t8567: F, t8995: F, t28166: F, t32110: F, t7732: F, t1353: F, t7933: F, t1907: F, t7311: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t125384 = t13426 * t8460;
    let t125385 = 2.0 * t125384;
    let t125386 = t18227 * t8460;
    let t125387 = 2.0 * t125386;
    let t125388 = t27123 * t8460;
    let t125389 = 2.0 * t125388;
    let t125390 = t28219 * t8460;
    let t125391 = 2.0 * t125390;
    let t125428 = t4147 * t28019;
    let t125478 = t8567 * t8995;
    let t125496 = t8567 * t28166;
    let t125507 = 2.0 * t7732 * t32110;
    let t125559 = t7933 * t1353;
    let t125563 = t1907 * t7311;
    (t125385, t125387, t125389, t125391, t125428, t125478, t125496, t125507, t125559, t125563)
}
