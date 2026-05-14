//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1024/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1024<F: Float>(t16026: F, t7378: F, t16022: F, t2551: F, t979: F, t1919: F, t1920: F, t6941: F, t695: F, t1060: F, t4265: F, t7375: F, t15197: F, t7383: F, t11885: F, t11891: F, t11894: F, t11900: F, t1470: F, t16935: F, t18081: F, t1883: F, t1888: F, t2543: F, t3077: F, t4625: F, t4659: F, t5231: F, t6278: F, t7035: F, t7051: F, t7340: F) -> (F,) {
    let t18124 = t7378 * t16026;
    let t18127 = t7378 * t16022;
    let t18132 = t979 * t2551;
    let t18142 = t1919 * t1920;
    let t18147 = t6941 * t695;
    let t18149 = t1919 * t18147 * t1060;
    let t18155 = 0.35374814814814814814e-1 * t4265 * t7375;
    let t18156 = t15197 * t7383;
    let t18160 = 0.53062222222222222222e-1 * t6278 * t18124 - 0.21224888888888888888e0 * t18081 * t18127 - 0.1857375e-1 * t11900 * t7051 + 0.5895802469135802469e-2 * t18132 - t11885 + 0.123825e-1 * t2543 * t4659 + 0.46434375e-2 * t2543 * t4625 + 0.1857375e-1 * t7340 * t1883 - 0.123825e-1 * t7340 * t1888 + 0.53062222222222222222e-1 * t3077 * t18142 - 0.26531111111111111111e-1 * t11891 + 0.17687407407407407407e-1 * t11894 - 0.53062222222222222222e-1 * t1470 * t18149 - 0.1857375e-1 * t5231 * t16935 - t18155 - 0.88437037037037037036e-1 * t18156 - 0.1857375e-1 * t11900 * t7035;
    (t18160,)
}
