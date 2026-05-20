//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3731/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3731<F: Float>(t21439: F, t3624: F, t1248: F, t21082: F, t1250: F, t12832: F, t13053: F, t17391: F, t17396: F, t17401: F, t17602: F, t17656: F, t17724: F, t21300: F, t3629: F, t3631: F, t3718: F, t3720: F, t3723: F, t5348: F, t56879: F, t57040: F, t57569: F, t59066: F, t69839: F, t70794: F, t70800: F, t70806: F, t70809: F, t70811: F) -> (F, F) {
    let t70819 = t21439 * t3624;
    let t70824 = t21082 * t1248;
    let t70830 = F::cast_from(0.17149607247227894789e-2_f64) * t59066 * t69839 * t13053 * t17656 - F::cast_from(0.28582678745379824648e-3_f64) * t56879 * t69839 * t70794 * t3629 - F::cast_from(0.42874018118069736972e-3_f64) * t70800 * t3723 + F::cast_from(0.45732285992607719436e-2_f64) * t17396 * t17724 + F::cast_from(0.76220476654346199061e-3_f64) * t70806 + F::cast_from(0.6351706387862183255e-4_f64) * t70809 + F::cast_from(0.11433071498151929859e-2_f64) * t70811 - F::cast_from(0.85748036236139473944e-3_f64) * t57040 * t5348 - F::cast_from(0.85748036236139473944e-3_f64) * t17401 * t17391 - F::cast_from(0.42874018118069736972e-3_f64) * t17401 * t17602 - F::cast_from(0.28582678745379824648e-3_f64) * t70819 * t3631 - F::cast_from(0.42874018118069736972e-3_f64) * t12832 * t21300 - F::cast_from(0.42874018118069736972e-3_f64) * t3718 * t3720 * t70824 * t1250 - F::cast_from(0.3811023832717309953e-3_f64) * t57569;
    (t70824, t70830)
}
