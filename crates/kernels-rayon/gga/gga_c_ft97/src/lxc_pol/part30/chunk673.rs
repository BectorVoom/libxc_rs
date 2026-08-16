//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 673/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk673(t28761: f64, t6317: f64, t24976: f64, t28516: f64, t24974: f64, t24987: f64, t28722: f64, t28727: f64, t28732: f64, t28739: f64, t28744: f64, t28749: f64, t28753: f64, t28758: f64) -> (f64, f64, f64) {
    let t28762 = t6317 * t28761;
    let t28764 = t24976 * t28516;
    let t28765 = t6317 * t28764;
    let t28767 = -t28722 / 3.0_f64 - t24974 / 36.0_f64 - t28727 / 36.0_f64 - t28732 / 36.0_f64 - 2.0_f64 / 9.0_f64 * t24987 - t28739 / 8.0_f64 - t28744 / 6.0_f64 + t28749 / 18.0_f64 + t28753 / 18.0_f64 - t28758 / 9.0_f64 - t28762 / 9.0_f64 - t28765 / 9.0_f64;
    (t28762, t28765, t28767)
}
