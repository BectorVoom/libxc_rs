//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 266/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk266(t3746: f64, t683: f64, t3051: f64, t2401: f64, t2402: f64, t3738: f64, t3741: f64, t3744: f64, t200: f64, t680: f64, t2379: f64, t3733: f64) -> (f64, f64, f64, f64, f64) {
    let t3747 = t683 * t3746;
    let t3748 = t3051 * t3747;
    let t3750 = t2401 + t2402 / 9.0_f64 + t3738 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t3741 + 2.0_f64 / 3.0_f64 * t3744 + 2.0_f64 / 3.0_f64 * t3748;
    let t3751 = t3750 * t200;
    let t3752 = t680 * t3751;
    let t3755 = t2379 * t3733;
    (t3748, t3750, t3751, t3752, t3755)
}
