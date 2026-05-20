//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3116/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3116<F: Float>(t81904: F, t81917: F, t81931: F, t81944: F, t81957: F, t81969: F, t81983: F, t81995: F, t1180: F, t1187: F, t1188: F, t12553: F, t17023: F, t17032: F, t20537: F, t20615: F, t20619: F, t20678: F, t24375: F, t24376: F, t24408: F, t3491: F, t45064: F, t45188: F, t45190: F, t5158: F, t5180: F, t58242: F, t6538: F, t81591: F, t81593: F, t81596: F, t81599: F, t81601: F, t81604: F) -> (F, F) {
    let t81998 = t81904 + t81917 + t81931 + t81944 + t81957 + t81969 + t81983 + t81995;
    let t82006 = F::cast_from(0.30762056574649219974e4_f64) * t12553 * t20678 * t5180 + F::cast_from(0.91082604192152556044e5_f64) * t45188 * t24375 * t45190 * t1187 - t81591 + t81593 + t81596 - t81599 + t81601 + t81604 + F::cast_from(0.17544670867903938621e1_f64) * t5158 * t20537 + F::cast_from(0.51947577317044391276e2_f64) * t58242 * t6538 - F::cast_from(0.10389515463408878255e3_f64) * t45064 * t24376 + F::cast_from(0.5848223622634646207e0_f64) * t3491 * t24408 + F::cast_from(0.5848223622634646207e0_f64) * t1180 * t81998 * t1188 - F::new(6.0) * t17023 * t20615 + F::cast_from(0.96491876992155210402e2_f64) * t17032 * t20619;
    (t81998, t82006)
}
