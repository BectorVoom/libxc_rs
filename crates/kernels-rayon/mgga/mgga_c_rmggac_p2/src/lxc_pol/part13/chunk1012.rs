//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1012/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1012(t117: f64, t29933: f64, t2295: f64, t40906: f64, t8640: f64, t2038: f64, t39116: f64, t7756: f64, t7933: f64, t2049: f64, t35688: f64, t7760: f64) -> (f64, f64, f64, f64) {
    let t42161 = t29933 * t117;
    let t42162 = t42161 * t2295;
    let t42166 = t8640 * t40906;
    let t42170 = t7933 * t2038 * t39116 * t7756;
    let t42174 = t35688 * t2049 * t39116 * t7760;
    (t42162, t42166, t42170, t42174)
}
