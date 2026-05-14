//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1072/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1072<F: Float>(t94588: F, t12844: F, t27583: F, t27585: F, t7978: F, t94904: F, t7968: F, t95006: F, t94934: F, t27575: F, t7974: F, t27651: F, t7964: F, t1598: F, t251: F, t40541: F) -> (F, F, F, F, F, F, F, F, F) {
    let t95088 = 0.51588271604938271604e-3 * t94588;
    let t95115 = t27583 * t12844 * t27585;
    let t95123 = t7978 * t94904;
    let t95125 = t7968 * t94904;
    let t95127 = t7978 * t95006;
    let t95130 = t27583 * t94934;
    let t95135 = t27575 * t7974;
    let t95137 = t7964 * t27651;
    let t95143 = t40541 * t251 * t1598;
    (t95088, t95115, t95123, t95125, t95127, t95130, t95135, t95137, t95143)
}
