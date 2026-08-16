//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 533/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk533(t5362: f64, t845: f64, t91: f64, t2823: f64, t4032: f64, t4049: f64, t5211: f64, t5215: f64, t5219: f64, t5223: f64, t5228: f64, t5302: f64, t5339: f64) -> (f64, f64) {
    let t5364 = t91 * t845 * t5362;
    let t5374 = -t5339 / 12.0_f64 + t5364 / 6.0_f64 + t2823 + 2.0_f64 / 27.0_f64 * t4032 + 2.0_f64 / 9.0_f64 * t4049 - 2.0_f64 / 27.0_f64 * t5211 + 2.0_f64 / 9.0_f64 * t5215 + 2.0_f64 / 9.0_f64 * t5219 - t5223 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t5228 - t5302 / 3.0_f64;
    (t5364, t5374)
}
