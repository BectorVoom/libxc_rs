//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1107/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1107(t10068: f64, t10074: f64, t10080: f64, t10086: f64, t10092: f64, t10097: f64, t10487: f64, t10496: f64, t37031: f64, t42313: f64, t9614: f64, t10116: f64, t10196: f64, t37039: f64, t42320: f64, t42322: f64, t42323: f64, t42324: f64, t7909: f64, t9107: f64, t9631: f64, t9636: f64) -> (f64, f64) {
    let t48074 = -t9614 + t10487 + t10068 + t10074 - t10080 - t42313 - t37031 - t10496 - t10086 + t10092 + t10097;
    let t48080 = t7909 - t9631 - t42320 + 0.25538759935978703639e-4_f64 * t9107 + t42322 - t42323 - t42324 + t9636 + t10116 + t10196 + t37039;
    (t48074, t48080)
}
