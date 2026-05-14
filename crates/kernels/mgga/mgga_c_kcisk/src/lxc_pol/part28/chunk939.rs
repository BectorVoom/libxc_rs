//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 939/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk939<F: Float>(t22294: F, t6666: F, t5192: F, t15903: F, t16676: F, t6987: F, t6681: F, t10414: F, t8481: F, t6982: F, t2533: F, t6945: F, t415: F, t6697: F, t7069: F, t1873: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22295 = t6666 * t22294;
    let t22296 = t5192 * t22295;
    let t22297 = t15903 * t22296;
    let t22299 = t16676 * t6987;
    let t22301 = t16676 * t6681;
    let t22303 = t10414 * t8481;
    let t22305 = t16676 * t6982;
    let t22307 = t6945 * t2533;
    let t22308 = t415 * t22307;
    let t22310 = t6697 * t7069;
    let t22311 = t1873 * t22310;
    (t22295, t22297, t22299, t22301, t22303, t22305, t22308, t22310, t22311)
}
