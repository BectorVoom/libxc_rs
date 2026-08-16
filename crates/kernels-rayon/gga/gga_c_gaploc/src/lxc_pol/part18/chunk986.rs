//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 986/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk986(t10713: f64, t2580: f64, t2508: f64, t2530: f64, t2958: f64) -> (f64, f64, f64) {
    let t10714 = t2580 * t10713;
    let t10716 = 0.15381052460284448567e-1_f64 * t2508 * t10714;
    let t10717 = t2958 * t2530;
    (t10714, t10716, t10717)
}
