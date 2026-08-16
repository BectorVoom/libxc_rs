//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 455/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk455(t676: f64, t762: f64, t2629: f64, t73: f64, t853: f64, t820: f64, t843: f64, t849: f64, t212: f64, t27: f64, t225: f64, t816: f64) -> (f64, f64, f64, f64, f64) {
    let t2630 = t676 * t762;
    let t2632 = 0.10843581300301739842e-1_f64 * t2629 * t2630;
    let t2638 = t73 * t853;
    let t2652 = t820 * t849 * t843;
    let t2659 = t27 * t212;
    let t2661 = t816 * t2659 * t225;
    (t2630, t2632, t2638, t2652, t2661)
}
