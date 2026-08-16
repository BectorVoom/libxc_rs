//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 614/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk614<F: Float>(t5285: F, t735: F, t5284: F, t1934: F, t718: F, t41: F, t642: F, t5068: F, t4797: F, t719: F, t1935: F, t1755: F, t4972: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5286 = t735 * t5285;
    let t5287 = t5284 * t5286;
    let t5289 = t1934 * t718;
    let t5290 = t41 * t642;
    let t5291 = t5290 * t5068;
    let t5292 = t5289 * t5291;
    let t5294 = t719 * t4797;
    let t5295 = t735 * t5294;
    let t5296 = t1935 * t5295;
    let t5298 = t1755 * t4972;
    (t5286, t5287, t5289, t5290, t5291, t5292, t5294, t5295, t5296, t5298)
}
