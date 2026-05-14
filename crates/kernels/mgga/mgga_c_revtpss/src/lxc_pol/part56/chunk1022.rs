//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1022/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1022<F: Float>(t125566: F, t125938: F, t125942: F, t125948: F, t125950: F, t127302: F, t127305: F, t127313: F, t127332: F, t127335: F, t129407: F, t129414: F, t129416: F, t129418: F, t32107: F, t32109: F, t32112: F, t5787: F, t8463: F, t8967: F) -> (F,) {
    let t131362 = t5787 * t8967 + t125566 + t125938 + t125942 - t125948 - t125950 + t127302 + t127305 + t127313 + t127332 + t127335 - 4.0 * t129407 - 4.0 * t129414 - 4.0 * t129416 - 4.0 * t129418 - t32107 - t32109 - t32112 - t8463;
    (t131362,)
}
