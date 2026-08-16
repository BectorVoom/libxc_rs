//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1375/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1375(t114770: f64, t114773: f64, t114775: f64, t114779: f64, t114783: f64, t114785: f64, t114787: f64, t114790: f64, t114794: f64, t114803: f64, t114807: f64, t114814: f64, t114816: f64, t114823: f64, t116848: f64, t116861: f64, t1843: f64, t1911: f64, t2163: f64, t22747: f64, t30716: f64, t30959: f64, t508: f64, t569: f64, t5877: f64, t8233: f64) -> f64 {
    let t116865 = -t116848 * t508 + t116861 * t569 - 3.0_f64 * t1843 * t30716 + 3.0_f64 * t1911 * t30959 - t2163 * t22747 - 3.0_f64 * t5877 * t8233 + t114770 - t114773 + t114775 + t114779 + t114783 - t114785 - t114787 - t114790 + t114794 - t114803 + t114807 - t114814 - t114816 - t114823;
    t116865
}
