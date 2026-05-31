//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1005/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1005<F: Float>(t1346: F, t3943: F, t3946: F, t481: F, t1311: F, t3860: F, t11407: F, t1315: F, t3853: F, t3898: F, t3897: F, t465: F) -> (F, F, F, F, F, F, F) {
    let t11536 = F::cast_from(1.0_f64) / t3943 / t1346;
    let t11539 = F::cast_from(1.0_f64) / t3946 / t481;
    let t11543 = t1311 * t3860;
    let t11557 = F::cast_from(0.55403703703703703703e-1_f64) * t11407;
    let t11571 = t3853 * t1315;
    let t11576 = t1311 * t3898;
    let t11580 = F::cast_from(1.0_f64) / t3897 / t465;
    (t11536, t11539, t11543, t11557, t11571, t11576, t11580)
}
