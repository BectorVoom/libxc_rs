//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1079/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1079<F: Float>(t26753: F, t7719: F, t1020: F, t14443: F, t7705: F, t7703: F, t1095: F, t283: F) -> (F, F, F, F, F) {
    let t26754 = t26753 * t7719;
    let t26755 = t1020 * t26754;
    let t26757 = t14443 * t7705;
    let t26758 = t7703 * t26757;
    let t26760 = t1095 * t283;
    (t26754, t26755, t26757, t26758, t26760)
}
