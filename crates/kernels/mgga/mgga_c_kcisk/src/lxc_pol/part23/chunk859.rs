//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 859/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk859<F: Float>(t13399: F, t3913: F, t470: F, t382: F, t1218: F, t338: F, t3923: F, t408: F, t1219: F, t3729: F, t3936: F, t3959: F, t1299: F, t389: F, t3934: F, t1319: F, t4065: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13400 = 0.73697530864197530862e-3 * t13399;
    let t13401 = t3913 * t470;
    let t13406 = t3913 * t382;
    let t13435 = t1218 * t1218;
    let t13436 = 1.0 / t13435;
    let t13437 = t338 * t13436;
    let t13440 = 1.0 / t3923 / t408;
    let t13448 = t3729 * t1219;
    let t13472 = t3936 * t3959;
    let t13482 = t389 * t1299 * t3934;
    let t13485 = t4065 * t1319;
    (t13400, t13401, t13406, t13435, t13436, t13437, t13440, t13448, t13472, t13482, t13485)
}
