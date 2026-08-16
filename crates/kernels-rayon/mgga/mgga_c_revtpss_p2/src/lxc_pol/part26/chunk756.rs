//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 756/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk756(t3989: f64, t4014: f64, t1370: f64, t9697: f64, t9700: f64, t9705: f64, t9711: f64, t9712: f64, t9716: f64, t9725: f64, t9729: f64, t9735: f64, t9739: f64, t9742: f64, t9745: f64, t9748: f64, t9750: f64) -> f64 {
    let t9753 = t3989 * t4014;
    let t9755 = 7.0_f64 / 48.0_f64 * t9697 - t1370 * t9700 / 48.0_f64 - 0.42874018118069736972e-3_f64 * t9705 + t9711 - 0.91464571985215438873e-3_f64 * t9712 + 0.85748036236139473944e-4_f64 * t9716 + t9725 - t9729 - t9735 + 0.30492001685571196935e-4_f64 * t9739 - 35.0_f64 / 72.0_f64 * t9742 - 7.0_f64 / 16.0_f64 * t9745 - t9748 * t9750 / 4.0_f64 - 0.60023625365297631762e-1_f64 * t9753;
    t9755
}
