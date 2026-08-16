//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1462/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1462(t32099: f64, t33952: f64, t33966: f64, t33968: f64, t33974: f64, t33979: f64, t33997: f64, t34008: f64, t34012: f64, t34018: f64, t34023: f64, t35240: f64, t38876: f64, t38880: f64, t38881: f64, t39339: f64, t39342: f64, t39519: f64, t39523: f64, t39551: f64) -> f64 {
    let t39579 = t32099 - t38876 + t33952 + t33966 - t33968 - t33974 - t33979 + t33997 + t34008 + t34012 + t38880 - t38881 + t39523 - t34018 + t34023 + t39339 + 2.0_f64 * t39551 - t39342 - t39519 - t35240;
    t39579
}
