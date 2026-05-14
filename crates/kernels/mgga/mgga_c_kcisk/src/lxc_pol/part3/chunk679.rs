//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 679/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk679<F: Float>(t1824: F, t4658: F, t11393: F, t706: F, t1797: F, t180: F, t479: F, t574: F, t682: F, t695: F, t1060: F, t1648: F, t1825: F, t4684: F, t7055: F, t1814: F) -> (F, F, F, F, F, F, F, F) {
    let t11394 = t4658 * t1824;
    let t11395 = t11393 * t11394;
    let t11396 = t706 * t11395;
    let t11400 = t180 * t479 * t1797;
    let t11401 = t574 * t682;
    let t11402 = t11401 * t695;
    let t11403 = t1060 * t1648;
    let t11405 = t11402 * t11403 * t1824;
    let t11408 = t1825 * t4684;
    let t11409 = t7055 * t11408;
    let t11412 = t1814 * t1648;
    (t11394, t11395, t11396, t11400, t11405, t11408, t11409, t11412)
}
