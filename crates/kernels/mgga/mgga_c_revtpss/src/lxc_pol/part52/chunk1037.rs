//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1037/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1037<F: Float>(t120956: F, t125587: F, t1414: F, t828: F, t121019: F, t32284: F, t5700: F, t121018: F, t1399: F, t33962: F, t34230: F, t4075: F, t121116: F, t33930: F, t1389: F, t32282: F) -> (F, F, F, F, F, F) {
    let t125590 = t120956 * t1414 * t828 * t125587;
    let t125599 = t32284 * t121019 * t5700;
    let t125603 = t121018 * t121019 * t33962 * t1399;
    let t125609 = t34230 * t4075;
    let t125617 = t121116 * t33930;
    let t125625 = t32282 * t1389;
    (t125590, t125599, t125603, t125609, t125617, t125625)
}
