//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 962/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk962(t10279: f64, t10400: f64, t10552: f64, t10555: f64, t10636: f64, t10641: f64, t10643: f64, t14697: f64, t14701: f64, t14706: f64, t14946: f64, t14895: f64) -> (f64, f64) {
    let t14947 = 4.0_f64 * t14697 + 2.0_f64 * t14701 - 6.0_f64 * t14706 + t10552 - 8.0_f64 / 9.0_f64 * t10400 - t10555 - t10636 - 8.0_f64 / 27.0_f64 * t10279 + t10641 + t10643 - t14946;
    let t14949 = 4.0_f64 / 9.0_f64 * t14895;
    (t14947, t14949)
}
