//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1021/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1021<F: Float>(t33231: F, t7458: F, t1873: F, t5449: F, t2040: F, t127553: F, t22574: F, t24432: F, t1442: F, t33553: F, t5457: F, t8595: F) -> (F, F, F, F, F, F) {
    let t128516 = F::cast_from(4.0_f64) * t7458 * t33231;
    let t128521 = t5449 * t1873;
    let t128523 = F::cast_from(2.0_f64) * t128521 * t2040;
    let t128535 = F::cast_from(6.0_f64) * t22574 * t24432 * t127553;
    let t128537 = F::cast_from(2.0_f64) * t1442 * t33553;
    let t128539 = F::cast_from(2.0_f64) * t5457 * t8595;
    (t128516, t128521, t128523, t128535, t128537, t128539)
}
