//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1085/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1085(t13725: f64, t484: f64, t42811: f64, t42814: f64, t42815: f64, t42816: f64, t42817: f64, t42821: f64, t42822: f64, t42823: f64, t42824: f64, t47001: f64) -> f64 {
    let t47003 = t484 * t13725;
    let t47005 = -t42811 - t42814 + t42815 + t42816 - t42817 - 0.28455006635676149599e-1_f64 * t47001 - 0.15808337019820083111e-2_f64 * t47003 - t42821 - t42822 - t42823 + t42824;
    t47005
}
