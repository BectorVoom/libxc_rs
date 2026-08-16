//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1123/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1123(t27: f64, t2718: f64, t8484: f64, t25386: f64, t2487: f64, t31752: f64, t31753: f64, t826: f64, t231: f64, t886: f64, t31830: f64, t8478: f64, t8479: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t119776 = t8484 * t2718 * t27;
    let t119777 = t25386 * t119776;
    let t119778 = t119777 * t2487;
    let t119779 = 0.7437465841810202164e-4_f64 * t119778;
    let t119781 = t31752 * t31753 * t826;
    let t119783 = t231 * t886;
    let t119788 = t31830 * t119776;
    let t119789 = t119788 * t2487;
    let t119790 = 0.13223814266738539448e-3_f64 * t119789;
    let t119792 = t8478 * t8479 * t31753;
    (t119777, t119779, t119781, t119783, t119788, t119790, t119792)
}
