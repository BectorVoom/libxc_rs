//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3085/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3085<F: Float>(t24274: F, t698: F, t52011: F, t58027: F, t77513: F, t24271: F, t1134: F, t6449: F, t16851: F, t16854: F, t43888: F, t58153: F, t58165: F, t58543: F, t81242: F, t81245: F, t81489: F) -> (F, F, F, F, F, F) {
    let t81491 = t698 * t24274;
    let t81494 = t52011 * t58027 * t77513;
    let t81496 = t698 * t24271;
    let t81498 = t6449 * t1134;
    let t81499 = t16851 * t81498;
    let t81501 = t16854 * t81498;
    let t81506 = F::cast_from(0.99655555555555555555e0_f64) * t81242 - F::new(0.35876e1) * t81245 - F::new(0.49294e0) * t81489 - F::cast_from(0.32862666666666666666e0_f64) * t81491 + F::new(0.147882e1) * t81494 - F::cast_from(0.2434271604938271605e-1_f64) * t81496 + F::cast_from(0.427258125e1_f64) * t81499 - F::cast_from(0.230371875e0_f64) * t81501 + t58543 - F::cast_from(0.73028148148148148149e0_f64) * t58153 - F::cast_from(0.91285185185185185187e-1_f64) * t58165 - F::cast_from(0.31003950617283950618e0_f64) * t43888;
    (t81491, t81494, t81496, t81499, t81501, t81506)
}
