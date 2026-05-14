//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 855/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk855<F: Float>(t12998: F, t12974: F, t1173: F, t1337: F, t459: F, t1354: F, t1422: F, t306: F, t3529: F, t3530: F, t425: F, t1417: F, t3589: F, t3595: F, t3621: F, t3521: F, t3567: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13091 = 0.36793333333333333333e0 * t12998;
    let t13092 = 0.93932222222222222223e0 * t12974;
    let t13110 = 0.55403703703703703703e-1 * t12974;
    let t13129 = t1337 * t1173 * t459;
    let t13138 = t1422 * t1354;
    let t13148 = t3529 * t306 * t459;
    let t13153 = t3530 * t425;
    let t13179 = t1417 * t3589;
    let t13183 = t1417 * t3595;
    let t13190 = t1417 * t3621;
    let t13192 = t3521 * t3567;
    (t13091, t13092, t13110, t13129, t13138, t13148, t13153, t13179, t13183, t13190, t13192)
}
