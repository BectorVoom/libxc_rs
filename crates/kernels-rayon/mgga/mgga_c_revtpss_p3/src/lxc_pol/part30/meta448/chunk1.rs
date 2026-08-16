//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1712/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1712(t1246: f64, t17608: f64, t1252: f64, t12956: f64, t12999: f64, t13012: f64, t13015: f64, t13018: f64, t17589: f64, t17593: f64, t17602: f64, t17605: f64, t3631: f64, t3647: f64, t3711: f64, t3718: f64, t5279: f64, t5304: f64) -> f64 {
    let t17609 = t17608 * t1246;
    let t17614 = 0.28582678745379824648e-3_f64 * t3711 * t17589 + t17593 + 0.28582678745379824648e-3_f64 * t12956 * t5279 - t12999 / 432.0_f64 + t13012 / 648.0_f64 - t13015 / 864.0_f64 + t13018 / 648.0_f64 - 0.21437009059034868486e-3_f64 * t3718 * t17602 + 0.15244095330869239812e-2_f64 * t17605 * t3631 + 0.42874018118069736972e-3_f64 * t17609 * t1252 + 0.47637797908966374414e-3_f64 * t3647 * t5304;
    t17614
}
