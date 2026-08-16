//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1177/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1177(t898: f64, t9005: f64, t2764: f64, t2770: f64, t895: f64, t9016: f64, t224: f64, t227: f64, t9015: f64, t2718: f64, t2724: f64, t873: f64, t8913: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36429 = t9005 * t898;
    let t36436 = t2764 * t2770;
    let t36439 = t895 * t9016;
    let t36513 = t224 / t9015 / t227;
    let t36533 = t2718 * t2724;
    let t36543 = t8913 * t873;
    (t36429, t36436, t36439, t36513, t36533, t36543)
}
