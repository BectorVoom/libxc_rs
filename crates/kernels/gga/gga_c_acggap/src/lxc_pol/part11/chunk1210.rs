//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1210/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1210<F: Float>(t11883: F, t624: F, t560: F, t811: F, t10956: F, t1679: F, t467: F, t9099: F, t1680: F, t2166: F, t29953: F, t29955: F, t29958: F, t36587: F, t36592: F, t36593: F, t36601: F, t36605: F, t567: F, t7288: F, t8021: F, t9082: F, t9096: F, t9098: F) -> F {
    let t36610 = t624 * t11883;
    let t36611 = t560 * t811;
    let t36617 = F::cast_from(2.0_f64) * t1679 * t10956 * t467;
    let t36619 = F::cast_from(4.0_f64) * t1679 * t9099;
    let t36620 = -t1680 * t567 * t8021 - F::cast_from(2.0_f64) * t2166 * t567 * t9082 + F::cast_from(4.0_f64) * t36587 * t9096 * t9098 + F::cast_from(6.0_f64) * t36593 * t567 * t7288 - F::cast_from(6.0_f64) * t36610 * t36611 * t9096 - t29953 + t29955 + F::cast_from(6.0_f64) * t29958 + t36592 - t36601 + t36605 - t36617 + t36619;
    t36620
}
