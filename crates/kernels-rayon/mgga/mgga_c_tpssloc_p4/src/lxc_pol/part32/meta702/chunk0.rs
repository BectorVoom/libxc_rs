//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2199/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2199(t28002: f64, t6535: f64, t12725: f64, t7461: f64, t19456: f64, t25980: f64, t4028: f64, t7468: f64, t2314: f64, t28045: f64, t4034: f64, t5107: f64, t652: f64, t7467: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t97844 = 4.0_f64 * t28002 * t6535;
    let t97846 = 4.0_f64 * t12725 * t7461;
    let t97848 = 4.0_f64 * t19456 * t7461;
    let t97850 = 4.0_f64 * t4028 * t25980;
    let t97854 = 4.0_f64 * t12725 * t7468;
    let t97856 = 4.0_f64 * t2314 * t28045;
    let t97858 = 4.0_f64 * t4034 * t28045;
    let t97862 = 4.0_f64 * t652 * t5107 * t7467;
    (t97844, t97846, t97848, t97850, t97854, t97856, t97858, t97862)
}
