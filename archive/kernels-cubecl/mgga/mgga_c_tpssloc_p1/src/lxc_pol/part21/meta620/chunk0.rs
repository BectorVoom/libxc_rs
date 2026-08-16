//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2396/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2396<F: Float>(t246: F, t40167: F, t12126: F, t588: F, t39037: F, t522: F, t2221: F, t3826: F, t3824: F, t12132: F, t592: F, t3696: F) -> (F, F, F, F, F, F, F) {
    let t40168 = t40167 * t246;
    let t40221 = t588 * t12126;
    let t40224 = F::cast_from(840.0_f64) * t39037 * t522;
    let t40225 = t2221 * t3826;
    let t40227 = t2221 * t3824;
    let t40230 = F::cast_from(16.0_f64) * t592 * t12132;
    let t40231 = t2221 * t3696;
    (t40168, t40221, t40224, t40225, t40227, t40230, t40231)
}
