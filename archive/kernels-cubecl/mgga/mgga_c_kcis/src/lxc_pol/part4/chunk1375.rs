//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1375/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1375<F: Float>(t15844: F, t1354: F, t6114: F, t2084: F, t3938: F, t3919: F, t6117: F, t3947: F, t5613: F, t11539: F, t1919: F, t1911: F, t3944: F) -> (F, F, F, F, F, F, F, F) {
    let t17739 = F::cast_from(0.15476481481481481481e-2_f64) * t15844;
    let t17762 = t6114 * t1354;
    let t17765 = t2084 * t3938;
    let t17768 = t6117 * t3919;
    let t17771 = t5613 * t3947;
    let t17772 = t17771 * t1354;
    let t17775 = t6117 * t3938;
    let t17778 = t1919 * t11539;
    let t17779 = t17778 * t3919;
    let t17784 = t1911 * t3944;
    (t17739, t17762, t17765, t17768, t17772, t17775, t17779, t17784)
}
