//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1088/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1088<F: Float>(t1587: F, t2471: F, t2868: F, t35327: F, t39694: F, t39789: F, t39792: F, t39804: F, t43100: F, t43107: F, t43108: F, t43135: F, t43138: F, t43139: F, t43141: F, t45781: F, t45788: F, t45794: F, t739: F, t9383: F) -> (F, F) {
    let t48638 = t2471 * t1587;
    let t48641 = t43100 - F::new(0.15323255961587222184e-3) * t45781 - F::new(0.11974241701863808564e0) * t2868 * t9383 + F::new(0.43639458646792546768e0) * t39694 + t43107 - t43108 - F::new(0.5107751987195740728e-4) * t45788 - F::new(0.10909864661698136692e0) * t45794 - F::new(0.66211599834018861287e-4) * t35327 - t43135 - F::new(0.60975299583150056624e-3) * t39789 - F::new(0.78064147182743091554e-3) * t39792 - t43138 - t43139 - F::new(0.60975299583150056624e-3) * t39804 + t43141 - F::new(0.11974241701863808564e0) * t739 * t48638;
    (t48638, t48641)
}
