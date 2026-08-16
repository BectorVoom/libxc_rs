//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1384/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1384(t21258: f64, t3720: f64, t1222: f64, t1266: f64, t17629: f64, t21228: f64, t21234: f64, t21236: f64, t21239: f64, t21242: f64, t21246: f64, t21249: f64, t21252: f64, t21255: f64, t3625: f64, t3718: f64, t5381: f64, t5384: f64, t5397: f64) -> f64 {
    let t21259 = t3720 * t21258;
    let t21264 = -0.28582678745379824648e-3_f64 * t3625 * t21228 + t17629 / 648.0_f64 + 0.15879265969655458138e-3_f64 * t21234 + t1222 * t21236 / 108.0_f64 + t1222 * t21239 / 36.0_f64 + 0.15244095330869239812e-2_f64 * t21242 * t1266 + 0.42874018118069736972e-3_f64 * t5384 * t21246 + t21249 / 162.0_f64 - t21252 / 864.0_f64 - t21255 / 432.0_f64 - 0.42874018118069736972e-3_f64 * t3718 * t21259 - 0.28582678745379824648e-3_f64 * t5381 * t5397;
    t21264
}
