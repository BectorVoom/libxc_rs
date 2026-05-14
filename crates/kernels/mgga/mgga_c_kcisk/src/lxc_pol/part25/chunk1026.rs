//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1026/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1026<F: Float>(t17839: F, t17897: F, t17948: F, t18173: F, t1908: F, t2041: F, t7654: F, t2666: F, t5533: F, t15845: F, t15849: F, t15853: F, t15856: F, t15860: F, t15864: F, t15866: F, t15871: F, t15876: F, t15878: F, t15882: F, t15884: F, t15886: F, t15888: F, t15895: F, t15900: F, t15907: F, t15912: F) -> (F, F, F, F, F) {
    let t18175 = t17839 + t17897 + t17948 + t18173;
    let t18176 = t1908 * t18175;
    let t18179 = t7654 * t2041;
    let t18182 = t2666 * t5533;
    let t18203 = 0.38691203703703703704e-2 * t15845 + 0.23214722222222222222e-2 * t15849 + 0.23214722222222222222e-2 * t15853 + 0.11607361111111111111e-2 * t15856 + 0.61905925925925925924e-2 * t15860 - 0.41270617283950617282e-2 * t15864 - 0.41270617283950617282e-2 * t15866 + 0.38691203703703703704e-2 * t15871 - 0.11607361111111111111e-2 * t15876 + 0.15476481481481481481e-2 * t15878 + 0.77382407407407407407e-3 * t15882 - 0.23214722222222222222e-2 * t15884 + 0.15476481481481481481e-2 * t15886 - 0.46429444444444444444e-2 * t15888 + 0.46429444444444444443e-2 * t15895 - 0.30952962962962962962e-2 * t15900 - 0.30952962962962962962e-2 * t15907 + 0.92858888888888888888e-2 * t15912;
    (t18175, t18176, t18179, t18182, t18203)
}
