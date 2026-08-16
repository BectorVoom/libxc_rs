//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 767/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk767(t3200: f64, t8372: f64, t3255: f64, t487: f64, t379: f64, t1909: f64, t3183: f64, t8506: f64, t11604: f64, t3194: f64, t3193: f64, t1851: f64, t358: f64) -> (f64, f64, f64, f64, f64) {
    let t12030 = t8372 * t3200;
    let t12033 = t487 * t3255;
    let t12034 = t12033 * t379;
    let t12035 = t1909 * t12034;
    let t12038 = t8506 * t3183;
    let t12041 = t3194 * t11604;
    let t12042 = t3193 * t12041;
    let t12045 = t1851 * t358;
    (t12030, t12035, t12038, t12042, t12045)
}
