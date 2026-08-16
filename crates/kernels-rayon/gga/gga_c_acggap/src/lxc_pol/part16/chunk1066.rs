//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1066/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1066(t30046: f64, t30048: f64, t30051: f64, t30056: f64, t33876: f64, t33887: f64, t36833: f64, t38781: f64, t38787: f64, t38792: f64, t38796: f64, t38799: f64, t38801: f64, t38805: f64, t38810: f64, t38815: f64, t38817: f64) -> f64 {
    let t38819 = -0.21437009059034868486e-3_f64 * t38781 + 0.15724046144802076034e-3_f64 * t38787 - 0.10718504529517434243e-2_f64 * t38792 + t30046 + t30048 + t30051 + t30056 - t36833 - 0.18007087609589289529e-1_f64 * t33876 + 0.4584375e-1_f64 * t38796 + 0.305625e-1_f64 * t38799 - 0.31448092289604152068e-3_f64 * t38801 - 0.31448092289604152068e-3_f64 * t38805 - 0.31448092289604152068e-3_f64 * t38810 - 0.20965394859736101379e-3_f64 * t38815 - t33887 - 0.42874018118069736972e-3_f64 * t38817;
    t38819
}
