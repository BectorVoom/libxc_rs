//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1117/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1117<F: Float>(t140: F, t178: F, t31986: F, t190: F, t3073: F, t3069: F, t31948: F, t31950: F, t31953: F, t31956: F, t31958: F, t31961: F, t31964: F, t31968: F, t31971: F, t31974: F, t31978: F, t31981: F, t31984: F) -> (F, F, F, F) {
    let t31988 = t140 * t178 * t31986;
    let t31991 = t140 * t3073 * t190;
    let t31994 = t140 * t3069 * t190;
    let t31996 = 0.20833333333333333334e-1 * t31948 + 0.20833333333333333334e-1 * t31950 + 0.8041666666666666667e-2 * t31953 + 0.8041666666666666667e-2 * t31956 + 0.40208333333333333335e-2 * t31958 - 0.23280625000000000001e-2 * t31961 + 0.99491666666666666664e-2 * t31964 - 0.10416666666666666667e-1 * t31968 - 0.10416666666666666667e-1 * t31971 - 0.20833333333333333334e-1 * t31974 - 0.99491666666666666664e-2 * t31978 + 0.2653111111111111111e-1 * t31981 - 0.19898333333333333333e-1 * t31984 + 0.19898333333333333333e-1 * t31988 - 0.2653111111111111111e-1 * t31991 + 0.30952962962962962962e-1 * t31994;
    (t31988, t31991, t31994, t31996)
}
