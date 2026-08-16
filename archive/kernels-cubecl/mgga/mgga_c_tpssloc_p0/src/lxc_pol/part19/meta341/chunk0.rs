//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1215/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1215<F: Float>(t2678: F, t828: F, t786: F, t9569: F, t805: F, t2610: F, t9541: F, t10041: F, t2563: F, t776: F, t222: F, t39934: F) -> (F, F, F, F, F, F, F) {
    let t41078 = t2678 * t828;
    let t41083 = t9569 * t786;
    let t41084 = t41083 * t805;
    let t41086 = t9541 * t2610;
    let t41088 = t2563 * t10041;
    let t41090 = t776 * t2678;
    let t41096 = F::cast_from(455.0_f64) / F::cast_from(243.0_f64) * t39934 * t222;
    (t41078, t41083, t41084, t41086, t41088, t41090, t41096)
}
