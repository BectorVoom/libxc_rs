//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 433/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk433<F: Float>(t2626: F, t760: F, t123: F, t192: F, t676: F, t762: F, t820: F, t843: F, t849: F, t857: F, t212: F, t27: F, t225: F, t816: F) -> (F, F, F, F, F, F) {
    let t2628 = 0.11696447245269292414e1 * t760 * t2626;
    let t2629 = t192 * t123;
    let t2630 = t676 * t762;
    let t2632 = 0.10843581300301739842e-1 * t2629 * t2630;
    let t2652 = t820 * t849 * t843;
    let t2653 = t2652 * t857;
    let t2659 = t27 * t212;
    let t2661 = t816 * t2659 * t225;
    (t2628, t2630, t2632, t2652, t2653, t2661)
}
