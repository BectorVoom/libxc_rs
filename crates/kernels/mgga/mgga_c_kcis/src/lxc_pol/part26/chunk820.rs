//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 820/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk820<F: Float>(t1590: F, t609: F, t4313: F, t622: F, t1588: F, t4413: F, t12305: F, t1625: F, t4479: F, t1627: F, t629: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12856 = t1590 * t1590;
    let t12857 = F::new(1.0) / t12856;
    let t12858 = t609 * t12857;
    let t12861 = F::new(1.0) / t4313 / t622;
    let t12890 = t1588 * t4413;
    let t12915 = F::cast_from(0.51588271604938271604e-3_f64) * t12305;
    let t12933 = t1625 * t4479;
    let t12938 = t1627 * t1627;
    let t12939 = F::new(1.0) / t12938;
    let t12940 = t629 * t12939;
    (t12856, t12857, t12858, t12861, t12890, t12915, t12933, t12938, t12939, t12940)
}
