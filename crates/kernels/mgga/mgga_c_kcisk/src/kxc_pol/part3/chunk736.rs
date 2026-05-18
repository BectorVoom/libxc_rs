//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 736/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk736<F: Float>(t11402: F, t11403: F, t1824: F, t1825: F, t4684: F, t7055: F, t1648: F, t1814: F, t4658: F, t4629: F, t10459: F, t707: F) -> (F, F, F, F, F, F) {
    let t11405 = t11402 * t11403 * t1824;
    let t11408 = t1825 * t4684;
    let t11409 = t7055 * t11408;
    let t11412 = t1814 * t1648;
    let t11413 = t11412 * t4658;
    let t11414 = t4629 * t11413;
    let t11417 = t10459 * t707;
    (t11405, t11408, t11409, t11413, t11414, t11417)
}
