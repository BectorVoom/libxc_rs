//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1191/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1191(t41984: f64, t41987: f64, t41989: f64, t41991: f64, t41992: f64, t41996: f64, t42001: f64, t42005: f64, t42008: f64, t42015: f64, t42018: f64, t42022: f64) -> f64 {
    let t48010 = t41984 - t41987 - t41989 + t41991 + t41992 - t41996 - 0.29792074959875355558e-1_f64 * t42001 + t42005 + t42008 - 0.69017266717057349418e1_f64 * t42015 - t42018 - t42022;
    t48010
}
