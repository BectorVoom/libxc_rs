//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1384/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1384<F: Float>(t30762: F, t30765: F, t14626: F, t1562: F, t3410: F, t10348: F, t8158: F, t10497: F, t10590: F, t1549: F, t1625: F, t1646: F, t30752: F, t30754: F, t30757: F, t30760: F, t30768: F, t30771: F, t30774: F, t30778: F, t30780: F, t528: F) -> F {
    let t34535 = F::cast_from(0.25561950635947166452e0_f64) * t30762;
    let t34536 = F::cast_from(0.25561950635947166452e0_f64) * t30765;
    let t34541 = F::cast_from(0.30674340763136599741e1_f64) * t1562 * t14626 * t3410;
    let t34548 = F::cast_from(0.14300195980740170668e1_f64) * t8158 * t10348;
    let t34549 = -t30752 + t30754 + t30757 + t30760 + t34535 - t34536 - t30768 + t30771 + t30774 - t30778 + t30780 + F::cast_from(0.35750489951850426669e0_f64) * t1625 * t10497 - t34541 + F::cast_from(0.71500979903700853338e0_f64) * t1549 * t10497 - F::cast_from(0.71500979903700853338e0_f64) * t528 * t10590 * t1646 - t34548;
    t34549
}
