//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 852/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk852(t1286: f64, t1526: f64, t1527: f64, t2: f64, t32031: f64, t32043: f64, t342: f64, t343: f64, t34592: f64, t34596: f64, t34601: f64, t34607: f64, t6512: f64, t6517: f64, t7151: f64, t7152: f64) -> f64 {
    let t34612 = (-t34592 * t7152 / 6.0_f64 + t32031 + t1286 * t34596 / 18.0_f64 + t1286 * t6517 / 3.0_f64 - t7151 * t34601 / 6.0_f64 - t32043 - t1526 * t1527 * t6512 / 12.0_f64 - t342 * t343 * t34607 / 4.0_f64) * t2;
    t34612
}
