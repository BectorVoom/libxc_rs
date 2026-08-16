//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 934/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk934(t1250: f64, t6688: f64, t3720: f64, t1222: f64, t1261: f64, t1782: f64, t1808: f64, t3657: f64, t3684: f64, t3718: f64, t464: f64, t5358: f64, t5363: f64, t5366: f64, t5373: f64, t5379: f64, t5381: f64, t5391: f64, t6653: f64, t6659: f64, t6663: f64, t6667: f64, t6673: f64, t6679: f64, t6683: f64) -> (f64, f64, f64) {
    let t6689 = t6688 * t1250;
    let t6690 = t3720 * t6689;
    let t6694 = t1222 * t6653 / 216.0_f64 + t5373 * t1782 / 54.0_f64 - t1222 * t6659 / 288.0_f64 - t1222 * t6663 / 144.0_f64 - t5358 / 432.0_f64 + 11.0_f64 / 108.0_f64 * t6667 * t464 - t3657 - 0.28582678745379824648e-3_f64 * t5363 - t5366 / 54.0_f64 + 0.23818898954483187207e-3_f64 * t1261 * t6673 + 0.15244095330869239812e-2_f64 * t5391 * t1808 - 0.14291339372689912324e-3_f64 * t1261 * t6679 - 0.28582678745379824648e-3_f64 * t1261 * t6683 - 0.28582678745379824648e-3_f64 * t5381 * t1808 - t3684 - 0.42874018118069736972e-3_f64 * t3718 * t6690 - 0.19055119163586549765e-3_f64 * t5379;
    (t6689, t6690, t6694)
}
