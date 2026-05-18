//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1215/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1215<F: Float>(t11582: F, t38248: F, t38249: F, t1065: F, t2847: F, t11002: F, t3269: F, t3274: F, t5086: F, t97: F, t40296: F, t792: F) -> (F, F, F, F) {
    let t40587 = t38248 * t11582 * t38249;
    let t40589 = t1065 * t2847;
    let t40590 = t11002 * t40589;
    let t40592 = F::new(5.0) / F::new(8.0) * t3269 * t40590;
    let t40594 = t97 * t3274 * t5086;
    let t40595 = t40296 * t792;
    (t40587, t40592, t40594, t40595)
}
