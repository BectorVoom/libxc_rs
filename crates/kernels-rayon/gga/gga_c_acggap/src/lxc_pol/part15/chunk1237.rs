//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1237/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1237(t32668: f64, t32670: f64, t32671: f64, t32672: f64, t35176: f64, t35180: f64, t35184: f64, t35190: f64, t35198: f64, t35204: f64, t37428: f64, t37430: f64, t39746: f64, t39750: f64, t39756: f64, t39763: f64, t39765: f64, t39767: f64) -> f64 {
    let t41784 = t32668 + t32670 - t32671 + t32672 + 0.21437009059034868486e-3_f64 * t39746 + 0.21437009059034868486e-3_f64 * t39750 - 0.83861579438944405516e-3_f64 * t35176 + 0.10718504529517434243e-2_f64 * t39756 + 0.42874018118069736972e-3_f64 * t35180 - 0.83861579438944405517e-3_f64 * t35184 - t37428 + 0.94344276868812456205e-2_f64 * t35190 - t37430 + 0.75475421495049964964e-2_f64 * t35198 - 0.18868855373762491241e-1_f64 * t39763 - 0.85748036236139473944e-3_f64 * t39765 - 0.56606566121287473724e-2_f64 * t39767 - 0.27953859812981468505e-1_f64 * t35204;
    t41784
}
