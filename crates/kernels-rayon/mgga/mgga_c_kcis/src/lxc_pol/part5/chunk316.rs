//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 316/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk316(t1164: f64, t355: f64, t377: f64, t1094: f64, t373: f64, sigma0: f64) -> (f64, f64, f64, f64) {
    let t1165 = t1164 * t355;
    let t1166 = t1165 * sigma0;
    let t1167 = t1166 * t377;
    let t1169 = t373 * t1094;
    let t1170 = t1169 * sigma0;
    (t1166, t1167, t1169, t1170)
}
