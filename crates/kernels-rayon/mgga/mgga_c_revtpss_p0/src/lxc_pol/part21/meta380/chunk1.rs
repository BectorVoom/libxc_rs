//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1793/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1793(t1170: f64, t12233: f64, t12240: f64, t12242: f64, t12245: f64, t12251: f64, t12360: f64, t12363: f64, t12366: f64, t12379: f64, t12395: f64, t12408: f64, t12413: f64, t12417: f64, t12418: f64, t12423: f64, t3447: f64, t3472: f64, t3480: f64, t435: f64) -> f64 {
    let t12426 = -0.19751673498613801407e-1_f64 * t12379 - t12233 - t12240 - t12242 - t12245 + t12251 - t12360 + t12363 - t12366 + t12395 - 0.310907e-1_f64 * t12408 * t435 + t12413 - t12417 + 3.0_f64 * t12418 * t1170 + 3.0_f64 * t3447 * t3472 + 0.96491876992155210402e2_f64 * t12423 * t3480;
    t12426
}
