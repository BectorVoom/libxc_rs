//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1037/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1037(t1356: f64, t35707: f64, t35713: f64, t35717: f64, t35720: f64, t35724: f64, t35729: f64, t35742: f64, t35744: f64, t4041: f64, t40480: f64, t40506: f64, t40596: f64, t47030: f64, t47032: f64, t47037: f64, t47042: f64, t5267: f64, t5888: f64, t8800: f64, t884: f64, t9944: f64) -> f64 {
    let t47044 = -0.23948483403727617128e0_f64 * t884 * t8800 * t5267 - 0.23948483403727617128e0_f64 * t1356 * t40596 * t5888 + 0.30487649791575028314e-3_f64 * t35707 + t35713 + t35717 - 0.43368970657079495312e-4_f64 * t35720 - 0.43368970657079495312e-4_f64 * t35724 - t35729 + 0.15243824895787514157e-3_f64 * t35742 + 0.15243824895787514157e-3_f64 * t35744 + 0.11974241701863808564e0_f64 * t4041 * t9944 + t40480 - 0.53205749866622299248e-5_f64 * t47030 + 0.24829349937757072983e-4_f64 * t47032 - t40506 - 0.12769379967989351819e-3_f64 * t47037 - 0.31923449919973379548e-4_f64 * t47042;
    t47044
}
