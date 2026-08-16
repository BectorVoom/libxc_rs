//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1070/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1070(t136226: f64, t136229: f64, t144892: f64, t144895: f64, t144899: f64, t144904: f64, t144908: f64, t144912: f64, t144917: f64, t144919: f64, t144923: f64, t144926: f64, t144930: f64, t144933: f64, t144935: f64, t144941: f64) -> f64 {
    let t145824 = -t144892 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t144895 - 2.0_f64 * t144899 + 4.0_f64 / 9.0_f64 * t144904 - 2.0_f64 * t144908 + t144912 + t144917 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t144919 - 4.0_f64 / 9.0_f64 * t144923 + t144926 / 18.0_f64 - t136226 / 3.0_f64 + t144930 / 18.0_f64 - t144933 / 3.0_f64 - t144935 / 9.0_f64 + t136229 / 18.0_f64 - t144941 / 6.0_f64;
    t145824
}
