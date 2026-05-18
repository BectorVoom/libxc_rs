//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 968/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk968<F: Float>(t1116: F, t3251: F, t2633: F, t1088: F, t3245: F, t977: F, t278: F, t2835: F, t975: F, t119: F, t251: F, t85: F) -> (F, F, F, F, F, F) {
    let t10426 = t3251 * t1116;
    let t10443 = F::new(6.0) * t2633;
    let t10450 = t3245 * t1088;
    let t10461 = t977 * t977;
    let t10462 = F::new(1.0) / t10461;
    let t10463 = t278 * t10462;
    let t10466 = t975 * t2835;
    let t10470 = t85 * t119 * t251;
    (t10426, t10443, t10450, t10463, t10466, t10470)
}
