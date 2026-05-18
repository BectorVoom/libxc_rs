//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 816/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk816<F: Float>(t150: F, t358: F, t378: F, t3524: F, t458: F, t12302: F, t2102: F, t11034: F, t3499: F, t2: F, t9224: F, t11008: F) -> (F, F, F, F, F) {
    let t12812 = t378 * t150 * t358;
    let t12816 = F::new(2.0) / F::new(3.0) * t458 * t3524;
    let t12817 = t2102 * t12302;
    let t12820 = t3499 * t11034;
    let t12823 = t9224 * t2;
    let t12824 = t12823 * t11008;
    (t12812, t12816, t12817, t12820, t12824)
}
