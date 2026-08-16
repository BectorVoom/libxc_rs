//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1206/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1206(t32339: f64, t32340: f64, t32341: f64, t32342: f64, t33874: f64, t33876: f64, t36836: f64, t38781: f64, t38787: f64, t38792: f64, t38796: f64, t38799: f64, t38801: f64, t38805: f64, t38810: f64, t38815: f64, t38817: f64) -> f64 {
    let t41360 = -0.42874018118069736972e-3_f64 * t38781 + 0.31448092289604152069e-3_f64 * t38787 - 0.21437009059034868486e-2_f64 * t38792 + t32339 + t32340 + t32341 + t32342 - 0.21437009059034868486e-2_f64 * t33874 - 0.36014175219178579057e-1_f64 * t33876 + 0.916875e-1_f64 * t38796 + 0.61125e-1_f64 * t38799 - 0.62896184579208304138e-3_f64 * t38801 - 0.62896184579208304138e-3_f64 * t38805 - 0.62896184579208304138e-3_f64 * t38810 - 0.41930789719472202759e-3_f64 * t38815 - t36836 - 0.85748036236139473944e-3_f64 * t38817;
    t41360
}
