//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1265/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1265<F: Float>(t18210: F, t28834: F, t7978: F, t1598: F, t251: F, t54605: F, t98624: F, t27601: F, t28727: F, t98637: F, t27651: F, t8209: F) -> (F, F, F, F, F, F) {
    let t99494 = F::new(0.23168402777777777778e-3) * t7978 * t18210 * t28834;
    let t99497 = t54605 * t251 * t1598;
    let t99504 = F::new(0.15476481481481481481e-2) * t98624;
    let t99506 = F::new(0.61782407407407407408e-3) * t28727 * t27601;
    let t99512 = F::new(0.15476481481481481481e-2) * t98637;
    let t99524 = t8209 * t27651;
    (t99494, t99497, t99504, t99506, t99512, t99524)
}
