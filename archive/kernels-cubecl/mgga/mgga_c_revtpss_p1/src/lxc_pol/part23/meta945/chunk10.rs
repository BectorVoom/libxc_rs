//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3114/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3114<F: Float>(t81509: F, t81511: F, t81514: F, t81516: F, t81518: F, t81521: F, t81523: F, t81525: F, t81527: F, t81530: F, t81533: F, t81536: F) -> F {
    let t81983 = -F::cast_from(0.485484375e1_f64) * t81509 + F::cast_from(0.58258125e1_f64) * t81511 - F::cast_from(0.1294625e1_f64) * t81514 - F::cast_from(0.3883875e1_f64) * t81516 - F::cast_from(0.3883875e1_f64) * t81518 + F::cast_from(0.6189328125e-1_f64) * t81521 - F::cast_from(0.1237865625e0_f64) * t81523 + F::cast_from(0.247573125e0_f64) * t81525 + F::cast_from(0.247573125e0_f64) * t81527 + F::cast_from(0.82524375e-1_f64) * t81530 - F::cast_from(0.82785e-1_f64) * t81533 - F::cast_from(0.82785e-1_f64) * t81536;
    t81983
}
