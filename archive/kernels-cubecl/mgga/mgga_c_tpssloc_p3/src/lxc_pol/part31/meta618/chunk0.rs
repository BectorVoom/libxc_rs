//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1867/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1867<F: Float>(t12020: F, t1385: F, t1992: F, t22635: F, t6439: F, t28117: F, t81159: F, t1377: F, t6330: F, t26331: F, t26332: F, t5187: F) -> (F, F, F, F) {
    let t96910 = t1992 * t22635 * t12020 * t6439 * t1385;
    let t96920 = t81159 * t28117;
    let t96922 = t1377 * t6330;
    let t96925 = t26331 * t22635 * t96922 * t1385;
    let t96929 = t26331 * t22635 * t26332 * t5187;
    (t96910, t96920, t96925, t96929)
}
