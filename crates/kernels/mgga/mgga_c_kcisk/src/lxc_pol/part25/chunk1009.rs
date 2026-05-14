//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1009/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1009<F: Float>(t2591: F, t5307: F, t1871: F, t7399: F, t1937: F, t2567: F, t5294: F, t1935: F, t11730: F, t718: F, t7317: F, t11236: F, t5320: F, t7431: F, t11774: F, t16716: F, t7316: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t17859 = t5307 * t2591;
    let t17861 = t7399 * t1871;
    let t17862 = t17861 * sigma2;
    let t17863 = t17862 * t1937;
    let t17865 = t2567 * t5294;
    let t17866 = t1935 * t17865;
    let t17868 = t11730 * t718;
    let t17869 = t17868 * t7317;
    let t17871 = t11236 * t5320;
    let t17872 = t17871 * t7431;
    let t17874 = t11774 * t718;
    let t17875 = t7316 * t16716;
    (t17859, t17861, t17863, t17866, t17869, t17872, t17874, t17875)
}
