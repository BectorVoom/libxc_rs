//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1338/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1338(t10390: f64, t10482: f64, t10484: f64, t10486: f64, t10754: f64, t11132: f64, t11136: f64, t11138: f64, t11140: f64, t11564: f64, t32434: f64, t9: f64) -> f64 {
    let t32436 = -0.7171875e-1_f64 * t10390 + t10482 + t10484 + t10486 + t10754 - t11132 + t11136 - t11138 + t11140 - t11564 + t9 * t32434;
    t32436
}
