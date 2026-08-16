//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 460/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk460(t72: f64, t752: f64, t757: f64, t2492: f64, t2596: f64, t745: f64, t760: f64, t123: f64, t192: f64, t676: f64, t762: f64, t820: f64, t843: f64, t849: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2622 = t752 * t72;
    let t2623 = t2622 * t757;
    let t2626 = t2596 * t2492 * t745;
    let t2628 = 0.11696447245269292414e1_f64 * t760 * t2626;
    let t2629 = t192 * t123;
    let t2630 = t676 * t762;
    let t2632 = 0.10843581300301739842e-1_f64 * t2629 * t2630;
    let t2652 = t820 * t849 * t843;
    (t2623, t2626, t2628, t2630, t2632, t2652)
}
