//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 997/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk997<F: Float>(t12841: F, t1599: F, t1607: F, t3970: F, t1590: F, t609: F, t4313: F, t622: F, t1588: F, t4413: F, t12305: F, t1625: F, t4479: F) -> (F, F, F, F, F, F, F) {
    let t12842 = t1599 * t12841;
    let t12844 = t3970 * t1607;
    let t12856 = t1590 * t1590;
    let t12857 = F::cast_from(1.0_f64) / t12856;
    let t12858 = t609 * t12857;
    let t12861 = F::cast_from(1.0_f64) / t4313 / t622;
    let t12890 = t1588 * t4413;
    let t12915 = F::cast_from(0.51588271604938271604e-3_f64) * t12305;
    let t12933 = t1625 * t4479;
    (t12842, t12844, t12858, t12861, t12890, t12915, t12933)
}
