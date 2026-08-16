//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1113/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1113(t5053: f64, t4934: f64, t21337: f64, t21382: f64, t21309: f64, t4952: f64, t6: f64, t30852: f64, t65693: f64, t21333: f64, t39: f64, t4960: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88289 = t5053 * t5053;
    let t88294 = t4934 * t4934;
    let t88310 = t21337 * t21382;
    let t88314 = t21309 * t6 * t4952;
    let t88320 = t30852 * t65693;
    let t88337 = t4960 * t39 * t21333;
    (t88289, t88294, t88310, t88314, t88320, t88337)
}
