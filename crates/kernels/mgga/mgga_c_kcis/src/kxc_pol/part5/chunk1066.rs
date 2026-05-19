//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1066/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1066<F: Float>(t1628: F, t6220: F, t15844: F, t3947: F, t5613: F, t1911: F, t3944: F, t2072: F, t4355: F, t4330: F, t16144: F, t16048: F) -> (F, F, F, F, F, F, F, F) {
    let t17710 = t6220 * t1628;
    let t17739 = F::cast_from(0.15476481481481481481e-2_f64) * t15844;
    let t17771 = t5613 * t3947;
    let t17784 = t1911 * t3944;
    let t17797 = t2072 * t4355;
    let t17834 = t2072 * t4330;
    let t17847 = F::cast_from(0.27785333333333333334e0_f64) * t16144;
    let t17856 = F::cast_from(0.22954444444444444444e0_f64) * t16048;
    (t17710, t17739, t17771, t17784, t17797, t17834, t17847, t17856)
}
