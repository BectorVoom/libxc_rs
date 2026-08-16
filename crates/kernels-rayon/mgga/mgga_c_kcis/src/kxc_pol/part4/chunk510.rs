//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 510/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk510(t2415: f64, t706: f64, t124: f64, t691: f64, t138: f64, t86: f64, t66: f64, t747: f64, t119: f64, t85: f64) -> (f64, f64, f64, f64) {
    let t2416 = t2415 * t706;
    let t2421 = t691 * t124;
    let t2423 = t86 * t2421 * t138;
    let t2425 = t66 * t747;
    let t2427 = t86 * t2425 * t138;
    let t2429 = t85 * t119;
    (t2416, t2423, t2427, t2429)
}
