//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 990/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk990<F: Float>(t1056: F, t1322: F, t5670: F, t13504: F, t13485: F, t6200: F, t3935: F, t442: F, t6211: F, t3937: F, t3283: F, t6199: F, t2168: F, t3532: F, t3278: F, t1311: F, t963: F) -> (F, F, F, F, F, F, F, F) {
    let t20025 = t1056 * t1322;
    let t20026 = t5670 * t20025;
    let t20027 = t13504 * t20026;
    let t20034 = t13485 * t6200;
    let t20036 = 0.11993859144118211475e-1 * t3935 * t20034;
    let t20037 = t6211 * t442;
    let t20038 = t20037 * t1056;
    let t20039 = t3937 * t20038;
    let t20042 = t6199 * t3283;
    let t20043 = t3937 * t20042;
    let t20046 = t2168 * t3532;
    let t20047 = t20046 * t3278;
    let t20048 = t13504 * t20047;
    let t20052 = t963 * t1311;
    (t20025, t20026, t20027, t20036, t20039, t20043, t20048, t20052)
}
