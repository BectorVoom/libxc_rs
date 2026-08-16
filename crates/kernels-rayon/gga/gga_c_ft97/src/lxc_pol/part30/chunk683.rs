//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 683/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk683(t24974: f64, t24987: f64, t28722: f64, t28727: f64, t28732: f64, t28739: f64, t28744: f64, t28749: f64, t28753: f64, t28758: f64, t28762: f64, t28765: f64) -> f64 {
    let t28897 = -t28722 - t24974 / 12.0_f64 - t28727 / 12.0_f64 - t28732 / 12.0_f64 - 2.0_f64 / 3.0_f64 * t24987 - 3.0_f64 / 8.0_f64 * t28739 - t28744 / 2.0_f64 + t28749 / 6.0_f64 + t28753 / 6.0_f64 - t28758 / 3.0_f64 - t28762 / 3.0_f64 - t28765 / 3.0_f64;
    t28897
}
