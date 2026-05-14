//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 252/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk252<F: Float>(t836: F, t841: F, t218: F, t344: F, t675: F, t334: F, t824: F, t219: F, t826: F, t837: F, t839: F) -> (F, F, F, F, F, F) {
    let t842 = t841 * t836;
    let t845 = t218 * t675 * t344;
    let t846 = 0.82156666666666666667e-1 * t845;
    let t847 = t334 * t824;
    let t849 = t218 * t219 * t847;
    let t851 = 0.1898925e1 * t837 - t839 + 0.8969e0 * t826 + 0.3071625e0 * t842 - t846 + 0.24647e0 * t849;
    (t842, t845, t846, t847, t849, t851)
}
