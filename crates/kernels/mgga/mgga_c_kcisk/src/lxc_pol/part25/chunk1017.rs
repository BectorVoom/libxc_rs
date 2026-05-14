//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1017/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1017<F: Float>(t17156: F, t5290: F, t5289: F, t1934: F, t2532: F, t5291: F, t17111: F, t5322: F, t5321: F, t2585: F, t740: F, t1872: F, t5323: F, t17132: F, t7568: F, t7302: F) -> (F, F, F, F, F, F, F, F) {
    let t17966 = t5290 * t17156;
    let t17967 = t5289 * t17966;
    let t17969 = t1934 * t2532;
    let t17970 = t17969 * t5291;
    let t17972 = t5322 * t17111;
    let t17973 = t5321 * t17972;
    let t17975 = t740 * t2585;
    let t17976 = t1872 * t17975;
    let t17977 = t17976 * t5323;
    let t17979 = t7568 * t17132;
    let t17980 = t7302 * t17979;
    (t17966, t17967, t17970, t17972, t17973, t17977, t17979, t17980)
}
