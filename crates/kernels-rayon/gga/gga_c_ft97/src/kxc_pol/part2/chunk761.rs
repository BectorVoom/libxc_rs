//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 761/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk761(t11416: f64, t11395: f64, t11399: f64, t11404: f64, t11408: f64, t11413: f64, t11783: f64, t11787: f64, t11791: f64, t11949: f64, t8260: f64, t11928: f64, t11936: f64, t11948: f64) -> f64 {
    let t11957 = 4.0_f64 / 3.0_f64 * t11416;
    let t11958 = -t11949 - t8260 - t11783 / 4.0_f64 + 3.0_f64 / 8.0_f64 * t11787 - t11791 / 2.0_f64 - t11395 - 4.0_f64 / 3.0_f64 * t11399 + 22.0_f64 / 9.0_f64 * t11404 + 2.0_f64 * t11408 + 4.0_f64 * t11413 - t11957;
    let t11960 = t11928 + t11936 + t11948 + t11958;
    t11960
}
