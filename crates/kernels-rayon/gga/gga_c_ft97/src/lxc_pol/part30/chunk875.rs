//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 875/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk875(t33867: f64, t33960: f64, t33977: f64, t35861: f64, t35866: f64, t35975: f64, t35979: f64, t35983: f64, t35987: f64, t35991: f64, t35995: f64, t35999: f64) -> f64 {
    let t36001 = t33867 + t35861 / 18.0_f64 + t35866 / 3.0_f64 - t35975 / 6.0_f64 - t33960 - 2.0_f64 / 9.0_f64 * t35979 - 2.0_f64 * t35983 + 4.0_f64 / 3.0_f64 * t35987 + t33977 + t35991 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t35995 - t35999 / 3.0_f64;
    t36001
}
