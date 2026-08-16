//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1198/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1198<F: Float>(t27583: F, t94934: F, t27575: F, t7974: F, t27651: F, t7964: F, t1598: F, t251: F, t40541: F, t27591: F, t27607: F, t2257: F, t2259: F, t44682: F) -> (F, F, F, F, F, F) {
    let t95130 = t27583 * t94934;
    let t95135 = t27575 * t7974;
    let t95137 = t7964 * t27651;
    let t95143 = t40541 * t251 * t1598;
    let t95157 = t27607 * t27591;
    let t95168 = F::cast_from(0.12871334876543209877e-3_f64) * t2257 * t44682 * t2259;
    (t95130, t95135, t95137, t95143, t95157, t95168)
}
