//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1041/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1041(t38704: f64, t38710: f64, t38712: f64, t34757: f64, t37214: f64, t38699: f64, t38702: f64, t38708: f64, t38717: f64, t38719: f64, t38724: f64, t38728: f64, t38733: f64, t38739: f64, t38742: f64, t38747: f64, t38749: f64, t4601: f64, t9318: f64) -> f64 {
    let t42712 = 0.35754263910370185096e-3_f64 * t38704;
    let t42714 = 0.47672351880493580128e-3_f64 * t38710;
    let t42715 = 0.11918087970123395032e-3_f64 * t38712;
    let t42728 = -0.1702583995731913576e-4_f64 * t38699 + 0.1702583995731913576e-4_f64 * t38702 + t42712 - 0.15323255961587222184e-3_f64 * t38708 - t42714 - t42715 - 0.5107751987195740728e-4_f64 * t38717 - 0.5107751987195740728e-4_f64 * t38719 + 0.1702583995731913576e-4_f64 * t38724 - 0.5107751987195740728e-4_f64 * t38728 - 0.1440846329149835838e-2_f64 * t38733 + 0.35922725105591425692e0_f64 * t4601 * t9318 - 0.17961362552795712846e0_f64 * t38739 - 0.8980681276397856423e-1_f64 * t38742 - t37214 - 0.32326021979378162576e-5_f64 * t34757 - 0.16364796992547205038e0_f64 * t38747 + 0.30487649791575028312e-3_f64 * t38749;
    t42728
}
