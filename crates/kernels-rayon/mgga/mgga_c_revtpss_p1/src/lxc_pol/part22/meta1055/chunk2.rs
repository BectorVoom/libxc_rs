//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3732/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3732(t12772: f64, t21160: f64, t3625: f64, t12784: f64, t12910: f64, t13312: f64, t17459: f64, t17461: f64, t17644: f64, t20747: f64, t20795: f64, t21040: f64, t21228: f64, t21298: f64, t3626: f64, t3628: f64, t3720: f64, t44459: f64, t44466: f64, t44609: f64, t5340: f64, t5351: f64, t5405: f64, t57147: f64, t57584: f64, t57586: f64, t57590: f64, t57602: f64) -> f64 {
    let t70857 = t3625 * t12772 * t21160;
    let t70872 = -0.25724410870841842184e-2_f64 * t44609 * t3720 * t20747 * t5405 - 0.28582678745379824648e-3_f64 * t3625 * t3626 * t21040 * t17644 - 0.28582678745379824648e-3_f64 * t5340 * t3626 * t20795 * t44466 - 0.57165357490759649296e-3_f64 * t5340 * t3626 * t20795 * t44459 - 0.91464571985215438872e-2_f64 * t57147 * t17461 - 0.57165357490759649296e-3_f64 * t12784 * t21228 - 0.3811023832717309953e-3_f64 * t70857 + 0.31758531939310916275e-3_f64 * t57584 - 0.28582678745379824648e-3_f64 * t3625 * t3626 * t5351 * t3628 * t13312 + 0.85748036236139473944e-3_f64 * t12910 * t3720 * t21298 * t17459 - 0.57165357490759649296e-3_f64 * t57586 - 0.28582678745379824648e-3_f64 * t57590 - 0.15244095330869239812e-2_f64 * t57602;
    t70872
}
