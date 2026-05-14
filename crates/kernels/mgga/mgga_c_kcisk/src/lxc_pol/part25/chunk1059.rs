//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1059/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1059<F: Float>(t17949: F, t17952: F, t17954: F, t17956: F, t17958: F, t17962: F, t17964: F, t17967: F, t17970: F, t17973: F, t17977: F, t17980: F, t17984: F, t17987: F, t17989: F, t18165: F, t18168: F, t18171: F) -> (F,) {
    let t18921 = 0.26979166666666666666e-1 * t17949 + 0.375e0 * t17952 - 0.125e0 * t17954 + 0.101171875e-1 * t17956 + 0.53958333333333333333e-1 * t17958 - 0.101171875e-1 * t17962 + 0.14388888888888888889e0 * t17964 - 0.625e-1 * t17967 + 0.33333333333333333334e0 * t17970 + 0.13489583333333333333e-1 * t17973 - 0.14388888888888888889e0 * t17977 + 0.27777777777777777777e-1 * t17980 + 0.1875e0 * t17984 - 0.89930555555555555554e-2 * t17987 - 0.125e0 * t17989 + 0.9375e-1 * t18165 + 0.13489583333333333333e-1 * t18168 + 0.20833333333333333333e-1 * t18171;
    (t18921,)
}
