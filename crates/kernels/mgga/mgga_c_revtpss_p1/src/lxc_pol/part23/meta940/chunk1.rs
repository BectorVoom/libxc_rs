//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3088/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3088<F: Float>(t141: F, t3417: F, t81186: F, t81509: F, t81511: F, t81514: F, t81516: F, t81518: F, t81521: F, t81523: F, t81525: F, t81527: F, t81530: F, t81533: F) -> (F, F) {
    let t81536 = t141 * t3417 * t81186;
    let t81538 = -F::cast_from(0.3560484375e1_f64) * t81509 + F::cast_from(0.427258125e1_f64) * t81511 - F::new(0.9494625e0) * t81514 - F::new(0.28483875e1) * t81516 - F::new(0.28483875e1) * t81518 + F::cast_from(0.1151859375e0_f64) * t81521 - F::cast_from(0.230371875e0_f64) * t81523 + F::new(0.46074375e0) * t81525 + F::new(0.46074375e0) * t81527 + F::new(0.15358125e0) * t81530 - F::cast_from(0.82156666666666666668e-1_f64) * t81533 - F::cast_from(0.82156666666666666668e-1_f64) * t81536;
    (t81536, t81538)
}
