//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 790/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk790(t432: f64, t4495: f64, t110: f64, t1871: f64, t492: f64, t452: f64, t488: f64, t3266: f64, t986: f64, t3214: f64, t3238: f64, t10969: f64, t3219: f64) -> (f64, f64, f64, f64, f64) {
    let t16261 = t4495 * t432;
    let t16263 = t1871 * t110 * t16261;
    let t16266 = t4495 * t492;
    let t16268 = t452 * t488 * t16266;
    let t16272 = t1871 * t986 * t3266;
    let t16276 = t452 * t3238 * t3214;
    let t16279 = t10969 * t3219;
    (t16263, t16268, t16272, t16276, t16279)
}
