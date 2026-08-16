//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 620/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk620(t2389: f64, t3683: f64, t774: f64, t3610: f64, t801: f64, t2142: f64, t2144: f64, t2147: f64, t2170: f64, t2173: f64, t2381: f64, t2384: f64, t3615: f64, t3618: f64, t3622: f64, t3626: f64, t3632: f64, t3635: f64, t3638: f64, t3667: f64, t3671: f64, t3678: f64, t3681: f64, t761: f64, t771: f64, t797: f64) -> (f64, f64, f64) {
    let t3685 = t2389 * t774 * t3683;
    let t3689 = t801 * t774 * t3610;
    let t3692 = t2142 + 7.0_f64 / 144.0_f64 * t2144 + 7.0_f64 / 144.0_f64 * t3615 + t2147 * t3618 / 16.0_f64 - t761 * t3622 / 48.0_f64 + t3626 * t3632 / 1536.0_f64 + 7.0_f64 / 4608.0_f64 * t3635 + t2173 * t3638 / 768.0_f64 - t771 * t3667 / 3072.0_f64 - t2173 * t3671 / 3072.0_f64 + 7.0_f64 / 4608.0_f64 * t2170 + t2381 + 7.0_f64 / 1152.0_f64 * t2384 + t2173 * t3678 / 768.0_f64 + 7.0_f64 / 1152.0_f64 * t3681 + 5.0_f64 / 768.0_f64 * t797 * t3685 - t797 * t3689 / 768.0_f64;
    (t3685, t3689, t3692)
}
