//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 159/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk159(t24: f64, t432: f64, t469: f64, t460: f64, t462: f64, t466: f64, t92: f64, t457: f64, t91: f64, t353: f64, t366: f64, t435: f64) -> (f64, f64, f64, f64, f64) {
    let t471 = t24 * t469 * t432;
    let t473 = -t460 - t462 * t466 / 3.0_f64 - t92 * t471;
    let t475 = t91 * t457 * t473;
    let t477 = t353 / 9.0_f64;
    let t480 = t475 / 6.0_f64 - t477 - t366 / 9.0_f64 - t435 / 3.0_f64;
    (t471, t473, t475, t477, t480)
}
