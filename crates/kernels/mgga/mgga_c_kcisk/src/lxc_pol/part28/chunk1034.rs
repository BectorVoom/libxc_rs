//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1034/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1034<F: Float>(t604: F, t2364: F, t23819: F, t11179: F, t1060: F, t2464: F, t6763: F, t5015: F, t6758: F, t10802: F, t22919: F, t1783: F, t1310: F, t17327: F, t17330: F, t17335: F, t17351: F, t17356: F, t17360: F, t17362: F, t1773: F, t5013: F) -> (F, F, F, F, F) {
    let t659 = 0.0 < t604;
    let t23820 = t2364 * t23819;
    let t23821 = t11179 * t23820;
    let t23824 = t2464 * t1060;
    let t23825 = t6763 * t23824;
    let t23826 = t5015 * t23825;
    let t23829 = t6758 * t23824;
    let t23830 = t10802 * t23829;
    let t23834 = piecewise3(t659, t22919, -t22919);
    let t23835 = t1783 * t23834;
    let t23836 = t1310 * t23835;
    let t23839 = -0.799590609607880765e-2 * t17327 + t17330 - t17335 - t17351 + 0.71963154864709268852e-1 * t5013 * t23821 - t17356 + t17360 - t17362 + 0.71963154864709268852e-1 * t5013 * t23826 - 0.47975436576472845901e-1 * t5013 * t23830 - 0.5397236614853195164e-1 * t1773 * t23836;
    (t23820, t23825, t23829, t23834, t23839)
}
