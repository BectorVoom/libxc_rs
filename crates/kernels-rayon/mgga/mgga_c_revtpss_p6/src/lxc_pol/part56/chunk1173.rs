//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1173/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1173(t125566: f64, t125938: f64, t125942: f64, t125948: f64, t125950: f64, t127302: f64, t127305: f64, t127313: f64, t127332: f64, t127335: f64, t129407: f64, t129414: f64, t129416: f64, t129418: f64, t32107: f64, t32109: f64, t32112: f64, t5787: f64, t8463: f64, t8967: f64) -> f64 {
    let t131362 = t5787 * t8967 + t125566 + t125938 + t125942 - t125948 - t125950 + t127302 + t127305 + t127313 + t127332 + t127335 - 4.0_f64 * t129407 - 4.0_f64 * t129414 - 4.0_f64 * t129416 - 4.0_f64 * t129418 - t32107 - t32109 - t32112 - t8463;
    t131362
}
