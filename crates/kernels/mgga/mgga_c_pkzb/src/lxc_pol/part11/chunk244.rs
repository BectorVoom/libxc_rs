//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 244/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk244<F: Float>(t12: F, t135: F, t273: F, t661: F, t687: F, t727: F, t729: F, t734: F, t803: F, t805: F, t439: F, t204: F, t334: F, t648: F, zeta_threshold: F) -> (F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t808 = t135 * t273 * t803 * t805 - t661 + t687 + t727 + t729 - t734;
    let t810 = piecewise3::<F>(t84, F::new(0.0), t439);
    let t819 = t204 * t648 * t334;
    (t808, t810, t819)
}
