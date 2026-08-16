//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 760/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk760(t11069: f64, t11041: f64, t11048: f64, t11052: f64, t11056: f64, t11061: f64, t11066: f64, t11073: f64, t11774: f64, t11939: f64, t8454: f64, t11076: f64) -> (f64, f64) {
    let t11946 = 2.0_f64 / 3.0_f64 * t11069;
    let t11948 = -6.0_f64 * t11041 - t11939 - 2.0_f64 / 3.0_f64 * t11048 - 2.0_f64 * t11052 - 2.0_f64 / 3.0_f64 * t11056 + 4.0_f64 / 3.0_f64 * t11061 + t11774 / 2.0_f64 - t8454 - 4.0_f64 / 3.0_f64 * t11066 + t11946 - 2.0_f64 / 3.0_f64 * t11073;
    let t11949 = 4.0_f64 / 9.0_f64 * t11076;
    (t11948, t11949)
}
