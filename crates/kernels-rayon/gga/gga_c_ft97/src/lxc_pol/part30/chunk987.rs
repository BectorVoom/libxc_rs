//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 987/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk987(t140535: f64, t140556: f64, t149743: f64, t149748: f64, t149750: f64, t149753: f64, t149760: f64, t149764: f64, t2354: f64, t27971: f64, t28010: f64, t28015: f64, t33269: f64, t33502: f64, t33504: f64, t3746: f64, t6005: f64, t6745: f64, t96834: f64) -> f64 {
    let t149766 = -t140535 / 9.0_f64 - 24.0_f64 * t96834 * t27971 - t149743 * t6005 / 18.0_f64 + t6745 * t33269 / 3.0_f64 + 4.0_f64 * t149748 + t149750 / 27.0_f64 + 4.0_f64 * t149753 + t140556 / 27.0_f64 + 2.0_f64 / 9.0_f64 * t28010 * t2354 * t33502 * t3746 + t149760 / 54.0_f64 - t28015 * t33504 / 9.0_f64 - 4.0_f64 * t149764;
    t149766
}
