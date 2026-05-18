//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 455/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk455<F: Float>(t676: F, t762: F, t2629: F, t73: F, t853: F, t820: F, t843: F, t849: F, t212: F, t27: F, t225: F, t816: F) -> (F, F, F, F, F) {
    let t2630 = t676 * t762;
    let t2632 = F::new(0.10843581300301739842e-1) * t2629 * t2630;
    let t2638 = t73 * t853;
    let t2652 = t820 * t849 * t843;
    let t2659 = t27 * t212;
    let t2661 = t816 * t2659 * t225;
    (t2630, t2632, t2638, t2652, t2661)
}
