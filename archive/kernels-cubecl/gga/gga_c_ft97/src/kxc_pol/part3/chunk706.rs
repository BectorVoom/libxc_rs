//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 706/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk706<F: Float>(t12362: F, t12365: F, t12571: F, t3541: F, t376: F, t89: F, t1882: F, t3452: F, t3457: F, t157: F, t1985: F, t1017: F, t604: F) -> (F, F, F, F, F, F, F, F) {
    let t12913 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t12362;
    let t12914 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12365;
    let t12918 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t12571;
    let t12963 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t89 * t376 * t3541;
    let t12965 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1882 * t3452;
    let t12967 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3457;
    let t12968 = t1985 * t157;
    let t12969 = t604 * t1017;
    (t12913, t12914, t12918, t12963, t12965, t12967, t12968, t12969)
}
