//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 879/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk879<F: Float>(t227: F, t3288: F, t967: F, t10447: F, t565: F, t1862: F, t4594: F, t1336: F, t140: F, t6677: F, t2563: F, t4644: F, t5184: F, t5182: F, t7409: F, t1802: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t15783 = 1.0 / t3288 / t227;
    let t15821 = 2.0 * t967;
    let t15822 = 6.0 * t10447;
    let t15823 = -t15821 + t15822;
    let t15824 = piecewise3(t228, 0.0, t15823);
    let t15825 = t565 * t15824;
    let t15842 = t4594 * t1862;
    let t15844 = t140 * t1336 * t15842;
    let t15845 = t15844 * t6677;
    let t15847 = t2563 * t4644;
    let t15848 = t5184 * t15847;
    let t15849 = t5182 * t15848;
    let t15851 = t7409 * sigma2;
    let t15852 = t15851 * t1802;
    (t15783, t15824, t15825, t15845, t15847, t15849, t15851, t15852)
}
