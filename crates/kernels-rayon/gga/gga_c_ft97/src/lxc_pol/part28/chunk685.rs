//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 685/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk685(t167: f64, t2185: f64, t26950: f64, t609: f64, t6615: f64, t574: f64, t605: f64, t1359: f64, t3590: f64, t1017: f64, t5975: f64, t26768: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26952 = t2185 * t167 * t26950;
    let t26955 = t6615 * t609;
    let t26957 = t574 * t605 * t26955;
    let t26961 = t574 * t3590 * t1359;
    let t26965 = t574 * t5975 * t1017;
    let t26969 = t574 * t167 * t26768;
    (t26952, t26955, t26957, t26961, t26965, t26969)
}
