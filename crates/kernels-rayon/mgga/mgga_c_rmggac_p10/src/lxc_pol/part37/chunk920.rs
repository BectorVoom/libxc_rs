//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 920/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk920(t76647: f64, t3351: f64, t3352: f64, t44187: f64, t515: f64, t44239: f64, t15457: f64, t16043: f64, t1971: f64, t2144: f64, t44232: f64, t44194: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76648 = 0.12769379967989351819e-4_f64 * t76647;
    let t76651 = t3351 * t3352 * t515 * t44187;
    let t76652 = 0.12769379967989351819e-4_f64 * t76651;
    let t76655 = t3351 * t3352 * t515 * t44239;
    let t76656 = 0.12769379967989351819e-4_f64 * t76655;
    let t76657 = t16043 * t15457;
    let t76658 = 0.12769379967989351819e-4_f64 * t76657;
    let t76661 = t3351 * t1971 * t2144 * t44232;
    let t76662 = 0.12769379967989351819e-4_f64 * t76661;
    let t76665 = t3351 * t1971 * t2144 * t44194;
    (t76648, t76652, t76656, t76658, t76662, t76665)
}
