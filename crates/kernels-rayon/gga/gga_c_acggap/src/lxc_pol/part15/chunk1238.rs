//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1238/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1238(t31039: f64, t32677: f64, t35238: f64, t35240: f64, t35244: f64, t37437: f64, t37438: f64, t37442: f64, t37443: f64, t37450: f64, t37451: f64, t39771: f64, t39775: f64, t39779: f64, t39782: f64, t39784: f64, t39786: f64, t39790: f64) -> f64 {
    let t41797 = -0.42874018118069736972e-3_f64 * t39771 - 0.42874018118069736972e-3_f64 * t39775 - t37437 - 0.42874018118069736972e-3_f64 * t39779 - 0.28582678745379824648e-3_f64 * t39782 + 0.64025200389650807211e-1_f64 * t39784 - 0.17149607247227894789e-1_f64 * t39786 + t37438 + t37442 + t37443 - 0.42874018118069736972e-2_f64 * t35238 - 0.25724410870841842184e-1_f64 * t35240 + 0.17149607247227894789e-2_f64 * t35244 - 0.94344276868812456204e-2_f64 * t39790 + t37450 - t37451 + t32677 + 0.80031500487063509014e-2_f64 * t31039;
    t41797
}
