//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 724/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk724(t32032: f64, t356: f64, t461: f64, t5700: f64, t342: f64, t630: f64, t7155: f64, t5617: f64, t72: f64, t1286: f64, t1526: f64, t1527: f64, t2: f64, t32026: f64, t32031: f64, t343: f64, t5692: f64, t5697: f64, t7151: f64, t7152: f64) -> (f64, f64, f64, f64, f64) {
    let t32033 = t356 * t32032;
    let t32038 = t461 * t5700;
    let t32043 = t342 * t630 * t7155 / 12.0_f64;
    let t32047 = t72 * t5617;
    let t32052 = (-t32026 * t7152 / 6.0_f64 + t32031 + t1286 * t32033 / 18.0_f64 + t1286 * t5697 / 3.0_f64 - t7151 * t32038 / 6.0_f64 - t32043 - t1526 * t1527 * t5692 / 12.0_f64 - t342 * t343 * t32047 / 4.0_f64) * t2;
    (t32033, t32038, t32043, t32047, t32052)
}
