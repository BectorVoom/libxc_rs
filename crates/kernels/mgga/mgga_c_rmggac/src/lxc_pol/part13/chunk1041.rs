//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1041/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1041<F: Float>(t38704: F, t38710: F, t38712: F, t34757: F, t37214: F, t38699: F, t38702: F, t38708: F, t38717: F, t38719: F, t38724: F, t38728: F, t38733: F, t38739: F, t38742: F, t38747: F, t38749: F, t4601: F, t9318: F) -> F {
    let t42712 = F::new(0.35754263910370185096e-3) * t38704;
    let t42714 = F::new(0.47672351880493580128e-3) * t38710;
    let t42715 = F::new(0.11918087970123395032e-3) * t38712;
    let t42728 = -F::new(0.1702583995731913576e-4) * t38699 + F::new(0.1702583995731913576e-4) * t38702 + t42712 - F::new(0.15323255961587222184e-3) * t38708 - t42714 - t42715 - F::new(0.5107751987195740728e-4) * t38717 - F::new(0.5107751987195740728e-4) * t38719 + F::new(0.1702583995731913576e-4) * t38724 - F::new(0.5107751987195740728e-4) * t38728 - F::new(0.1440846329149835838e-2) * t38733 + F::new(0.35922725105591425692e0) * t4601 * t9318 - F::new(0.17961362552795712846e0) * t38739 - F::new(0.8980681276397856423e-1) * t38742 - t37214 - F::new(0.32326021979378162576e-5) * t34757 - F::new(0.16364796992547205038e0) * t38747 + F::new(0.30487649791575028312e-3) * t38749;
    t42728
}
