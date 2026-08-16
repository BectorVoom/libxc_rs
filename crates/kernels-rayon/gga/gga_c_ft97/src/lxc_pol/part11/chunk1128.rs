//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1128/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1128(t41499: f64, t41502: f64, t41505: f64, t41508: f64, t41516: f64, t41519: f64, t41522: f64, t41525: f64, t41528: f64, t41531: f64, t41540: f64, t43631: f64) -> f64 {
    let t43639 = -0.17780800291358024693e0_f64 * t41499 + 0.88904001456790123462e-1_f64 * t41502 + 0.1333560021851851852e0_f64 * t41505 - 0.1333560021851851852e0_f64 * t41508 - t43631 + 0.16669500273148148149e-1_f64 * t41516 + 0.2469555596021947874e-1_f64 * t41519 - 0.22226000364197530866e-1_f64 * t41522 - 0.29634667152263374488e-1_f64 * t41525 + 0.69147556688614540471e-1_f64 * t41528 + 0.22226000364197530865e-1_f64 * t41531 + 0.17286889172153635117e0_f64 * t41540;
    t43639
}
