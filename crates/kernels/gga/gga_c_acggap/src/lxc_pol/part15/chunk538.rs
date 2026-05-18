//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 538/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk538<F: Float>(t3770: F, t390: F, t3055: F, t383: F, t1039: F, t1032: F, t993: F, t1103: F, t175: F, t3044: F, t398: F, t1036: F) -> (F, F, F, F, F, F) {
    let t3772 = F::new(0.60023625365297631762e-2) * t3770 * t390;
    let t3775 = t3055 * t383;
    let t3777 = F::new(0.12862205435420921092e-2) * t3775 * t1039;
    let t3782 = F::new(0.30011812682648815881e-2) * t1032 * t993;
    let t3793 = t1032 * t1103;
    let t3806 = t398 * t175 * t3044;
    let t3808 = F::new(0.12862205435420921092e-2) * t1036 * t3806;
    (t3772, t3777, t3782, t3793, t3806, t3808)
}
