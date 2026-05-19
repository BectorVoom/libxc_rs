//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 972/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk972<F: Float>(t1151: F, t3427: F, t3384: F, t1149: F, t3435: F, t3433: F, t1160: F, t3444: F, t1156: F, t3476: F, t1170: F, t12233: F, t12240: F, t12242: F, t12245: F, t12251: F, t12360: F, t12363: F, t12366: F, t12379: F, t12395: F, t12408: F, t3447: F, t3472: F, t3480: F, t435: F) -> (F, F, F) {
    let t12411 = t1151 * t3427;
    let t12413 = F::new(6.0) * t3384 * t12411;
    let t12415 = t3427 * t3435 * t1149;
    let t12417 = F::cast_from(0.48245938496077605201e2_f64) * t3433 * t12415;
    let t12418 = t3444 * t1160;
    let t12423 = t1156 * t3476;
    let t12426 = -F::cast_from(0.19751673498613801407e-1_f64) * t12379 - t12233 - t12240 - t12242 - t12245 + t12251 - t12360 + t12363 - t12366 + t12395 - F::new(0.310907e-1) * t12408 * t435 + t12413 - t12417 + F::new(3.0) * t12418 * t1170 + F::new(3.0) * t3447 * t3472 + F::cast_from(0.96491876992155210402e2_f64) * t12423 * t3480;
    (t12413, t12417, t12426)
}
