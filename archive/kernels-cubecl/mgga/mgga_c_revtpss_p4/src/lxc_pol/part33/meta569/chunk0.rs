//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1976/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1976<F: Float>(t1518: F, t8233: F, t1519: F, t2165: F, t29427: F, t29590: F, t29993: F, t29998: F, t30007: F, t30015: F, t30113: F, t30125: F, t30127: F, t30130: F, t30154: F, t30156: F, t30158: F, t30951: F, t30959: F, t4248: F, t569: F, t5887: F, t5921: F, t651: F, t6934: F, t7586: F, t8158: F) -> (F, F) {
    let t30963 = t8233 * t1518;
    let t30973 = -F::cast_from(4.0_f64) * t1519 * t29427 + t2165 * t6934 - F::cast_from(2.0_f64) * t30951 * t651 + t30959 * t569 - F::cast_from(4.0_f64) * t30963 * t651 - F::cast_from(4.0_f64) * t4248 * t8158 - F::cast_from(4.0_f64) * t5887 * t7586 - F::cast_from(2.0_f64) * t5921 * t7586 - t29590 - t29993 - t29998 - t30007 + t30015 + t30113 - t30125 - t30127 - t30130 - t30154 - t30156 - t30158;
    (t30963, t30973)
}
