//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 683/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk683<F: Float>(t10491: F, t2: F, t10478: F, t1775: F, t2772: F, t2775: F, t305: F, t631: F, t7242: F, t798: F, t898: F, t10246: F) -> (F, F, F, F, F, F) {
    let t10603 = t10491 * t2;
    let t10613 = t10478 * t2;
    let t10617 = t1775 * t2772;
    let t10619 = t1775 * t2775;
    let t10631 = F::new(1.0) / t305 / t631 / t898 / t798 / t7242 / F::new(4.0);
    let t10636 = F::new(2.0) / F::new(9.0) * t10246;
    (t10603, t10613, t10617, t10619, t10631, t10636)
}
