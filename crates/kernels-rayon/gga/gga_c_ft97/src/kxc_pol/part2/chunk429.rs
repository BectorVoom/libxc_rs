//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 429/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk429(t689: f64, t709: f64, t680: f64, t194: f64, t195: f64, t25: f64) -> (f64, f64, f64, f64) {
    let t2388 = t689 * t709;
    let t2389 = t680 * t2388;
    let t2393 = 1.0_f64 / t195 / t194;
    let t2394 = t25 * t2393;
    (t2388, t2389, t2393, t2394)
}
