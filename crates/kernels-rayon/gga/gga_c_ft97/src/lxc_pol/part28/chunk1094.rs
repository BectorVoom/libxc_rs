//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1094/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1094(t146143: f64, t146171: f64, t146182: f64, t146208: f64, t146218: f64, t146340: f64, t146473: f64, t146552: f64, t146593: f64, t146937: f64, t146972: f64, t22907: f64, t34352: f64, t379: f64, t5501: f64) -> f64 {
    let t147004 = -12.0_f64 * t146937 + 8.0_f64 * t146218 + 8.0_f64 * t146340 + 8.0_f64 * t146182 + 4.0_f64 * t146171 + 4.0_f64 * t146473 + 4.0_f64 * t146552 + 8.0_f64 * t146208 - 12.0_f64 * t146972 + 8.0_f64 * t146143 + 2.0_f64 / 9.0_f64 * t5501 * t22907 * t34352 * t379 + 4.0_f64 * t146593;
    t147004
}
