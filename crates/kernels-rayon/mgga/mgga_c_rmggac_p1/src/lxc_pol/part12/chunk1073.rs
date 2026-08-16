//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1073/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1073(t4601: f64, t9008: f64, t27036: f64, t681: f64, t26346: f64, t7710: f64, t117: f64, t29933: f64, t2295: f64, t40906: f64, t8640: f64, t2038: f64, t39116: f64, t7756: f64, t7933: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42151 = t4601 * t9008;
    let t42152 = 0.23948483403727617128e0_f64 * t42151;
    let t42156 = t27036 * t681;
    let t42159 = t26346 * t7710;
    let t42161 = t29933 * t117;
    let t42162 = t42161 * t2295;
    let t42166 = t8640 * t40906;
    let t42167 = 0.10909864661698136691e0_f64 * t42166;
    let t42170 = t7933 * t2038 * t39116 * t7756;
    (t42152, t42156, t42159, t42162, t42167, t42170)
}
