//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 813/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk813(t5542: f64, t8607: f64, t674: f64, t2004: f64, t7677: f64, t8571: f64, t34659: f64, t34662: f64, t34665: f64, t38312: f64, t38315: f64, t38318: f64, t38322: f64, t38326: f64, t38328: f64, t38336: f64, t38340: f64, t38344: f64, t38348: f64, t38352: f64, t5928: f64, t7704: f64) -> (f64, f64, f64) {
    let t38354 = t8607 * t5542;
    let t38355 = t38354 * t674;
    let t38356 = t38355 * t2004;
    let t38358 = t8571 * t7677;
    let t38360 = 0.81300399444200075504e-3_f64 * t38312 + t38315 - 0.33335697577410973224e-1_f64 * t38318 + 0.66671395154821946448e-1_f64 * t34659 - 0.1951603679568577289e-3_f64 * t38322 + 0.30487649791575028314e-3_f64 * t38326 - 0.5987120850931904282e-1_f64 * t38328 - 0.11974241701863808564e0_f64 * t5928 * t7704 + 0.29810146462873361018e-2_f64 * t34662 + 0.29810146462873361018e-2_f64 * t34665 + 0.15243824895787514157e-3_f64 * t38336 + 0.15243824895787514157e-3_f64 * t38340 + 0.30487649791575028314e-3_f64 * t38344 + 0.15243824895787514157e-3_f64 * t38348 - 0.85129199786595678796e-5_f64 * t38352 - 0.85129199786595678796e-5_f64 * t38356 - 0.42564599893297839398e-5_f64 * t38358;
    (t38354, t38355, t38360)
}
