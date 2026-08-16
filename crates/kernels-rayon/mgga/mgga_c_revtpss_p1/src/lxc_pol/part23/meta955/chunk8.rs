//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3190/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3190(t17448: f64, t21090: f64, t1248: f64, t1794: f64, t471: f64, t12916: f64, t24730: f64, t5340: f64, t12784: f64, t12787: f64, t17753: f64, t20800: f64, t20836: f64, t20941: f64, t24787: f64, t3625: f64, t3720: f64, t44521: f64, t5331: f64, t5333: f64, t5401: f64, t57660: f64, t59196: f64, t6421: f64, t69832: f64, t70890: f64, t71009: f64, t71020: f64, t82838: f64, t82886: f64) -> (f64, f64) {
    let t83783 = t17448 * t21090;
    let t83792 = t1794 * t1248 * t471;
    let t83798 = t5340 * t12916 * t24730;
    let t83808 = -0.85748036236139473947e-3_f64 * t44521 * t69832 * t5401 - 0.45732285992607719436e-2_f64 * t57660 * t20941 + 0.30488190661738479624e-2_f64 * t71009 - 0.57165357490759649296e-3_f64 * t71020 - 0.57165357490759649296e-3_f64 * t83783 - 0.42874018118069736972e-3_f64 * t12784 * t24787 + 0.7145669686344956162e-3_f64 * t3625 * t12787 * t6421 * t82838 + 0.64311027177104605458e-3_f64 * t17753 * t3720 * t70890 * t83792 + 0.85748036236139473947e-3_f64 * t83798 - 0.64311027177104605458e-3_f64 * t5331 * t3720 * t20800 * t20836 - 0.21437009059034868486e-3_f64 * t59196 * t3720 * t82886 * t5333;
    (t83792, t83808)
}
