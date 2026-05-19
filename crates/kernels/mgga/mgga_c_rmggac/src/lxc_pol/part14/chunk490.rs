//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 490/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk490<F: Float>(t571: F, t848: F, t5245: F, t552: F, t305: F, t326: F, t3839: F, t3851: F, t4669: F, t4952: F, t5041: F, t5049: F, t5136: F, t5142: F, t5145: F, t5148: F, t5149: F, t5152: F, t5155: F, t5156: F, t5160: F, t5162: F, t5163: F, t5166: F, t5169: F, t5251: F, t5254: F, t5259: F, t5260: F, t5263: F, t5266: F, t5268: F, t5271: F, t797: F, t838: F) -> (F, F) {
    let t5274 = t571 * t848;
    let t5277 = t5245 * t552;
    let t5279 = F::cast_from(0.17961362552795712846e0_f64) * t797 * t5041 + F::cast_from(0.23948483403727617128e0_f64) * t838 * t5136 + F::cast_from(0.11974241701863808564e0_f64) * t305 * t4952 - F::cast_from(0.59871208509319042821e-1_f64) * t5142 - F::cast_from(0.35922725105591425692e0_f64) * t4669 * t5145 - F::cast_from(0.23948483403727617128e0_f64) * t5148 * t5149 + F::cast_from(0.35922725105591425692e0_f64) * t3851 * t5152 + F::cast_from(0.47896966807455234256e0_f64) * t5155 * t5156 + F::cast_from(0.59871208509319042821e-1_f64) * t5160 - F::cast_from(0.14369090042236570277e1_f64) * t5162 * t5163 - F::cast_from(0.35922725105591425692e0_f64) * t4669 * t5166 + F::cast_from(0.19957069503106347607e-1_f64) * t5251 + F::cast_from(0.11974241701863808564e0_f64) * t326 * t5254 + F::cast_from(0.11974241701863808564e1_f64) * t3839 * t5049 + F::cast_from(0.23948483403727617128e0_f64) * t5259 * t5260 - F::cast_from(0.35922725105591425692e0_f64) * t4669 * t5263 + F::cast_from(0.23948483403727617128e0_f64) * t5266 * t5268 + F::cast_from(0.71845450211182851384e0_f64) * t5271 * t5169 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t5274 - F::cast_from(0.11974241701863808564e0_f64) * t5277;
    (t5277, t5279)
}
