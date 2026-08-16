//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 435/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk435(t2413: f64, t683: f64, t92: f64, t2401: f64, t2402: f64, t2407: f64, t2411: f64) -> (f64, f64, f64) {
    let t2414 = t683 * t2413;
    let t2415 = t92 * t2414;
    let t2417 = t2401 + 2.0_f64 / 9.0_f64 * t2402 - 2.0_f64 / 9.0_f64 * t2407 + 2.0_f64 / 3.0_f64 * t2411 - t2415 / 3.0_f64;
    (t2414, t2415, t2417)
}
