//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 617/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk617<F: Float>(t2783: F, t458: F, t8282: F, t849: F, t1775: F, t2778: F, t2767: F, t303: F, t3051: F, t1771: F, t854: F, t10491: F, t2: F, t10478: F, t2772: F, t2775: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10584 = t458 * t2783;
    let t10586 = t8282 * t849;
    let t10589 = t1775 * t2778;
    let t10591 = t1775 * t2767;
    let t10594 = 28.0 / 27.0 * t3051 * t303;
    let t10595 = t1771 * t854;
    let t10603 = t10491 * t2;
    let t10613 = t10478 * t2;
    let t10617 = t1775 * t2772;
    let t10619 = t1775 * t2775;
    (t10584, t10586, t10589, t10591, t10594, t10595, t10603, t10613, t10617, t10619)
}
