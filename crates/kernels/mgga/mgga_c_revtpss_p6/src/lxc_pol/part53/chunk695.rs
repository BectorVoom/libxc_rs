//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 695/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk695<F: Float>(t239: F, t7262: F, t820: F, t1401: F, t1405: F, t2019: F, t545: F, t64: F, t1416: F, t7251: F, t7253: F, t7258: F, t7261: F) -> (F, F, F, F, F) {
    let t7264 = t820 * t7262 * t239;
    let t7265 = t7264 * t1401;
    let t7267 = t2019 * t1405;
    let t7268 = F::cast_from(0.20007875121765877254e-2_f64) * t7267;
    let t7269 = t545 * t64;
    let t7271 = t820 * t7269 * t239;
    let t7272 = t7271 * t1416;
    let t7274 = -t7251 - t7253 / F::new(48.0) - t7258 + t7261 - F::cast_from(0.42874018118069736972e-3_f64) * t7265 - t7268 - F::cast_from(0.17149607247227894789e-2_f64) * t7272;
    (t7264, t7268, t7269, t7271, t7274)
}
