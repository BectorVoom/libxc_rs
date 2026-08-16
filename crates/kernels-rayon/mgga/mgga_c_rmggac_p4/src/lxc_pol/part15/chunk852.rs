//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 852/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk852(t42151: f64, t40906: f64, t8640: f64, t2038: f64, t39116: f64, t7756: f64, t7933: f64, t2049: f64, t35688: f64, t7760: f64, t1982: f64, t7428: f64, t8602: f64) -> (f64, f64, f64, f64, f64) {
    let t42152 = 0.23948483403727617128e0_f64 * t42151;
    let t42166 = t8640 * t40906;
    let t42167 = 0.10909864661698136691e0_f64 * t42166;
    let t42170 = t7933 * t2038 * t39116 * t7756;
    let t42174 = t35688 * t2049 * t39116 * t7760;
    let t42177 = t8602 * t7428 * t1982;
    (t42152, t42167, t42170, t42174, t42177)
}
