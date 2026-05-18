//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 602/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk602<F: Float>(t1102: F, t1697: F, t278: F, t344: F, t4597: F, t4603: F, t4608: F, t4627: F, t4630: F, t4634: F, t4639: F, t4644: F, t4672: F, t4768: F, t975: F) -> F {
    let t4771 = F::new(0.98556445e-3) * t1102 * t4597 + F::new(0.7391733375e-3) * t1102 * t4603 - F::new(0.1478346675e-2) * t1102 * t4608 + F::new(0.1478346675e-2) * t344 * t4627 - F::new(0.65704296666666666667e-3) * t4630 - F::new(0.65704296666666666667e-3) * t1102 * t4634 - F::new(0.1478346675e-2) * t1102 * t4639 + F::new(0.19711289e-2) * t1102 * t4644 - F::new(0.98556445e-3) * t344 * t4672 - F::new(4.0) * t975 * t1697 - F::new(4.0) * t278 * t4768;
    t4771
}
