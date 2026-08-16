//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 703/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk703(t2754: f64, t827: f64, t828: f64, t2695: f64, t2702: f64, t2704: f64, t2707: f64, t2716: f64, t2721: f64, t2726: f64, t2730: f64, t2732: f64, t2739: f64, t2742: f64, t2745: f64, t2751: f64, t799: f64, t825: f64) -> (f64, f64) {
    let t2756 = t827 * t828 * t2754;
    let t2759 = 0.57165357490759649296e-4_f64 * t2695 + t2702 + 7.0_f64 / 72.0_f64 * t2704 - t799 * t2707 / 48.0_f64 + t2716 + 0.42874018118069736972e-3_f64 * t2721 * t2726 + t2730 * t2732 / 16.0_f64 - t2739 + 0.20007875121765877254e-2_f64 * t2742 + 0.17149607247227894789e-2_f64 * t2745 * t2751 - 0.21437009059034868486e-3_f64 * t825 * t2756;
    (t2756, t2759)
}
