//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 819/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk819<F: Float>(t1354: F, t1422: F, t306: F, t3529: F, t459: F, t3530: F, t425: F, t12825: F, t458: F, t11313: F, t1425: F, t13009: F, t420: F, t1361: F, t3598: F, t12974: F) -> (F, F, F, F, F, F, F, F) {
    let t13138 = t1422 * t1354;
    let t13148 = t3529 * t306 * t459;
    let t13153 = t3530 * t425;
    let t13220 = t12825 * t458;
    let t13238 = t11313 * t1425;
    let t13244 = t13009 * t420;
    let t13247 = t3598 * t1361;
    let t13263 = 0.12841111111111111111e-1 * t12974;
    (t13138, t13148, t13153, t13220, t13238, t13244, t13247, t13263)
}
