//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2181/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2181(t2018: f64, t22129: f64, t807: f64, t22262: f64, t25986: f64, t2661: f64, t22182: f64, t94508: f64, t102486: f64, t102489: f64, t102495: f64, t94444: f64, t94460: f64, t98145: f64, t98147: f64, t98152: f64, t98157: f64) -> f64 {
    let t108554 = t807 * t2018 * t22129;
    let t108559 = t2661 * t25986 * t22262;
    let t108562 = t94508 * t22182;
    let t108564 = -0.28582678745379824648e-3_f64 * t108554 + 0.10841600599314203355e-2_f64 * t94444 - t102486 + t98145 + t98147 + t102489 - 0.57165357490759649295e-3_f64 * t98152 - t98157 + t102495 - 0.11433071498151929859e-3_f64 * t108559 - 0.11337795902333997111e-1_f64 * t94460 + 0.50820002809285328225e-4_f64 * t108562;
    t108564
}
