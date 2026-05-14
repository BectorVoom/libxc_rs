//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 998/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk998<F: Float>(t1311: F, t403: F, t442: F, t964: F, t3966: F, t6196: F, t2159: F, t4000: F, t1305: F, t6152: F, t1309: F, t13839: F, t13861: F, t13866: F, t13868: F, t13873: F, t13902: F, t13906: F, t13910: F, t2170: F, t405: F) -> (F,) {
    let t20195 = t1311 * t403 * t442;
    let t20196 = t964 * t20195;
    let t20202 = 0.35981577432354634426e-1 * t3966 * t6196;
    let t20203 = t2159 * t4000;
    let t20206 = t6152 * t1305;
    let t20214 = 0.35981577432354634426e-1 * t1309 * t20196 + 0.28785261945883707542e0 * t13839 * t2170 - t20202 + 0.52772980234120130494e0 * t20203 * t405 - 0.95950873152945691804e-1 * t20206 - 0.2398771828823642295e-1 * t13861 - 0.95950873152945691802e-1 * t13866 + 0.63967248768630461201e-1 * t13868 + t13873 - 0.79959060960788076501e-2 * t13902 + 0.11993859144118211475e-1 * t13906 - 0.31983624384315230601e-1 * t13910;
    (t20214,)
}
