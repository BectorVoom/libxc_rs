//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 98/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk98(t287: f64, t291: f64, t286: f64, t285: f64) -> (f64, f64, f64, f64, f64) {
    let t292 = t287 * t291;
    let t293 = t286 * t292;
    let t296 = 1.0_f64 + t285 * t293 / 96.0_f64;
    let t297 = f64::ln(t296);
    let t299 = 1.0_f64 + 0.66725e-1_f64 * t297;
    let t300 = 1.0_f64 / t299;
    (t292, t293, t296, t299, t300)
}
