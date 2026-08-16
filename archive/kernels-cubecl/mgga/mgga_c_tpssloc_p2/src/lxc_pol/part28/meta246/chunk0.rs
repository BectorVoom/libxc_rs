//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1071/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1071<F: Float>(t334: F, t371: F, t28: F, t776: F, t868: F, t1271: F, t191: F, t192: F) -> (F, F, F, F, F) {
    let t6793 = t371 * t334;
    let t6841 = t28 * t776;
    let t6848 = t28 * t868;
    let t6875 = t1271 * t191;
    let t6876 = t6875 * t192;
    (t6793, t6841, t6848, t6875, t6876)
}
