//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1382/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1382(t30762: f64, t30765: f64, t14626: f64, t1562: f64, t3410: f64, t10348: f64, t8158: f64, t10497: f64, t10590: f64, t1549: f64, t1625: f64, t1646: f64, t30752: f64, t30754: f64, t30757: f64, t30760: f64, t30768: f64, t30771: f64, t30774: f64, t30778: f64, t30780: f64, t528: f64) -> f64 {
    let t34535 = 0.25561950635947166452e0_f64 * t30762;
    let t34536 = 0.25561950635947166452e0_f64 * t30765;
    let t34541 = 0.30674340763136599741e1_f64 * t1562 * t14626 * t3410;
    let t34548 = 0.14300195980740170668e1_f64 * t8158 * t10348;
    let t34549 = -t30752 + t30754 + t30757 + t30760 + t34535 - t34536 - t30768 + t30771 + t30774 - t30778 + t30780 + 0.35750489951850426669e0_f64 * t1625 * t10497 - t34541 + 0.71500979903700853338e0_f64 * t1549 * t10497 - 0.71500979903700853338e0_f64 * t528 * t10590 * t1646 - t34548;
    t34549
}
