//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 734/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk734<F: Float>(t3748: F, t8090: F, t8259: F, t3739: F, t8079: F, t1413: F, t8161: F, t8074: F, t8130: F, t960: F, t8133: F, t965: F, t8136: F, t8123: F, t970: F, t8126: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t25306 = t3748 * t8090;
    let t25308 = t8259 * sigma0;
    let t25327 = t3739 * t8079;
    let t25350 = t8161 * t1413;
    let t25351 = t25350 * sigma0;
    let t25376 = t3739 * t8074;
    let t25425 = t960 * t8130;
    let t25427 = t965 * t8133;
    let t25429 = t965 * t8136;
    let t25485 = t970 * t8123;
    let t25487 = t960 * t8126;
    (t25306, t25308, t25327, t25350, t25351, t25376, t25425, t25427, t25429, t25485, t25487)
}
