//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 737/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk737<F: Float>(t1620: F, t2347: F, t6311: F, t6314: F, t6319: F, t6324: F, t6326: F, t6329: F, t6334: F, t6337: F, t6341: F, t6345: F, t6347: F, t6349: F, t6352: F, t6355: F, t6358: F, t6361: F, t6364: F, t6366: F, t6371: F, t6374: F, t6378: F, t6380: F, t6383: F, t6385: F, t6389: F, t6391: F) -> (F, F, F) {
    let t6607 = t2347 * t1620;
    let t6623 = 0.9375e-1 * t6311 - 0.625e-1 * t6314 - 0.20833333333333333333e-1 * t6319 - 0.44965277777777777777e-2 * t6324 - 0.25e0 * t6326 + 0.13489583333333333333e-1 * t6329 + 0.1875e0 * t6334 - 0.53958333333333333333e-1 * t6337 + 0.101171875e-1 * t6341 - 0.101171875e-1 * t6345 + 0.53958333333333333333e-1 * t6347 + 0.625e-1 * t6349 - 0.16666666666666666667e0 * t6352;
    let t6637 = -0.13489583333333333333e-1 * t6355 - 0.625e-1 * t6358 + 0.25e0 * t6361 + 0.71944444444444444443e-1 * t6364 - 0.9375e-1 * t6366 - 0.20234375e-1 * t6371 + 0.13489583333333333333e-1 * t6374 - 0.9375e-1 * t6378 + 0.101171875e-1 * t6380 + 0.101171875e-1 * t6383 + 0.625e-1 * t6385 - 0.9375e-1 * t6389 - 0.13489583333333333333e-1 * t6391;
    (t6607, t6623, t6637)
}
