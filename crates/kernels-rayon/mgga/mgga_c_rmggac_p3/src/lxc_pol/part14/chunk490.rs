//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 490/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk490(t571: f64, t848: f64, t5245: f64, t552: f64, t305: f64, t326: f64, t3839: f64, t3851: f64, t4669: f64, t4952: f64, t5041: f64, t5049: f64, t5136: f64, t5142: f64, t5145: f64, t5148: f64, t5149: f64, t5152: f64, t5155: f64, t5156: f64, t5160: f64, t5162: f64, t5163: f64, t5166: f64, t5169: f64, t5251: f64, t5254: f64, t5259: f64, t5260: f64, t5263: f64, t5266: f64, t5268: f64, t5271: f64, t797: f64, t838: f64) -> (f64, f64) {
    let t5274 = t571 * t848;
    let t5277 = t5245 * t552;
    let t5279 = 0.17961362552795712846e0_f64 * t797 * t5041 + 0.23948483403727617128e0_f64 * t838 * t5136 + 0.11974241701863808564e0_f64 * t305 * t4952 - 0.59871208509319042821e-1_f64 * t5142 - 0.35922725105591425692e0_f64 * t4669 * t5145 - 0.23948483403727617128e0_f64 * t5148 * t5149 + 0.35922725105591425692e0_f64 * t3851 * t5152 + 0.47896966807455234256e0_f64 * t5155 * t5156 + 0.59871208509319042821e-1_f64 * t5160 - 0.14369090042236570277e1_f64 * t5162 * t5163 - 0.35922725105591425692e0_f64 * t4669 * t5166 + 0.19957069503106347607e-1_f64 * t5251 + 0.11974241701863808564e0_f64 * t326 * t5254 + 0.11974241701863808564e1_f64 * t3839 * t5049 + 0.23948483403727617128e0_f64 * t5259 * t5260 - 0.35922725105591425692e0_f64 * t4669 * t5263 + 0.23948483403727617128e0_f64 * t5266 * t5268 + 0.71845450211182851384e0_f64 * t5271 * t5169 - 0.59871208509319042821e-1_f64 * t326 * t5274 - 0.11974241701863808564e0_f64 * t5277;
    (t5277, t5279)
}
