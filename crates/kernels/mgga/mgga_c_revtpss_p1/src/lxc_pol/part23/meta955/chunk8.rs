//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3190/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3190<F: Float>(t17448: F, t21090: F, t1248: F, t1794: F, t471: F, t12916: F, t24730: F, t5340: F, t12784: F, t12787: F, t17753: F, t20800: F, t20836: F, t20941: F, t24787: F, t3625: F, t3720: F, t44521: F, t5331: F, t5333: F, t5401: F, t57660: F, t59196: F, t6421: F, t69832: F, t70890: F, t71009: F, t71020: F, t82838: F, t82886: F) -> (F, F) {
    let t83783 = t17448 * t21090;
    let t83792 = t1794 * t1248 * t471;
    let t83798 = t5340 * t12916 * t24730;
    let t83808 = -F::cast_from(0.85748036236139473947e-3_f64) * t44521 * t69832 * t5401 - F::cast_from(0.45732285992607719436e-2_f64) * t57660 * t20941 + F::cast_from(0.30488190661738479624e-2_f64) * t71009 - F::cast_from(0.57165357490759649296e-3_f64) * t71020 - F::cast_from(0.57165357490759649296e-3_f64) * t83783 - F::cast_from(0.42874018118069736972e-3_f64) * t12784 * t24787 + F::cast_from(0.7145669686344956162e-3_f64) * t3625 * t12787 * t6421 * t82838 + F::cast_from(0.64311027177104605458e-3_f64) * t17753 * t3720 * t70890 * t83792 + F::cast_from(0.85748036236139473947e-3_f64) * t83798 - F::cast_from(0.64311027177104605458e-3_f64) * t5331 * t3720 * t20800 * t20836 - F::cast_from(0.21437009059034868486e-3_f64) * t59196 * t3720 * t82886 * t5333;
    (t83792, t83808)
}
