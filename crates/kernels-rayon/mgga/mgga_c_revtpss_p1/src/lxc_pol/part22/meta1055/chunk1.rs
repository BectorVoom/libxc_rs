//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3731/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3731(t21439: f64, t3624: f64, t1248: f64, t21082: f64, t1250: f64, t12832: f64, t13053: f64, t17391: f64, t17396: f64, t17401: f64, t17602: f64, t17656: f64, t17724: f64, t21300: f64, t3629: f64, t3631: f64, t3718: f64, t3720: f64, t3723: f64, t5348: f64, t56879: f64, t57040: f64, t57569: f64, t59066: f64, t69839: f64, t70794: f64, t70800: f64, t70806: f64, t70809: f64, t70811: f64) -> (f64, f64) {
    let t70819 = t21439 * t3624;
    let t70824 = t21082 * t1248;
    let t70830 = 0.17149607247227894789e-2_f64 * t59066 * t69839 * t13053 * t17656 - 0.28582678745379824648e-3_f64 * t56879 * t69839 * t70794 * t3629 - 0.42874018118069736972e-3_f64 * t70800 * t3723 + 0.45732285992607719436e-2_f64 * t17396 * t17724 + 0.76220476654346199061e-3_f64 * t70806 + 0.6351706387862183255e-4_f64 * t70809 + 0.11433071498151929859e-2_f64 * t70811 - 0.85748036236139473944e-3_f64 * t57040 * t5348 - 0.85748036236139473944e-3_f64 * t17401 * t17391 - 0.42874018118069736972e-3_f64 * t17401 * t17602 - 0.28582678745379824648e-3_f64 * t70819 * t3631 - 0.42874018118069736972e-3_f64 * t12832 * t21300 - 0.42874018118069736972e-3_f64 * t3718 * t3720 * t70824 * t1250 - 0.3811023832717309953e-3_f64 * t57569;
    (t70824, t70830)
}
