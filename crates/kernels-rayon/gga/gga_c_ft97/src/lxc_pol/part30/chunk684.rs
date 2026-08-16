//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 684/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk684(t24995: f64, t25010: f64, t28770: f64, t28774: f64, t28779: f64, t28783: f64, t28784: f64, t28790: f64, t28794: f64, t28798: f64, t28802: f64, t28805: f64) -> f64 {
    let t28911 = -t28770 / 3.0_f64 + t28774 / 9.0_f64 - t28779 / 2.0_f64 - 3.0_f64 * t28783 - t28784 / 18.0_f64 + t24995 / 3.0_f64 - t25010 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t28790 - 6.0_f64 * t28794 + t28798 / 3.0_f64 - t28802 / 2.0_f64 - 2.0_f64 / 3.0_f64 * t28805;
    t28911
}
