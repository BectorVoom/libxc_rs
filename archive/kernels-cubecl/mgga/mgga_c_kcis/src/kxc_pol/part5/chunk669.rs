//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 669/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk669<F: Float>(t1102: F, t1697: F, t278: F, t344: F, t4597: F, t4603: F, t4608: F, t4627: F, t4630: F, t4634: F, t4639: F, t4644: F, t4672: F, t4768: F, t975: F) -> F {
    let t4771 = F::cast_from(0.98556445e-3_f64) * t1102 * t4597 + F::cast_from(0.7391733375e-3_f64) * t1102 * t4603 - F::cast_from(0.1478346675e-2_f64) * t1102 * t4608 + F::cast_from(0.1478346675e-2_f64) * t344 * t4627 - F::cast_from(0.65704296666666666667e-3_f64) * t4630 - F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t4634 - F::cast_from(0.1478346675e-2_f64) * t1102 * t4639 + F::cast_from(0.19711289e-2_f64) * t1102 * t4644 - F::cast_from(0.98556445e-3_f64) * t344 * t4672 - F::cast_from(4.0_f64) * t975 * t1697 - F::cast_from(4.0_f64) * t278 * t4768;
    t4771
}
