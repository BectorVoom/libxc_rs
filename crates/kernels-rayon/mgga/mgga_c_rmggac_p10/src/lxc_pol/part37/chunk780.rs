//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 780/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk780(t16503: f64, t3369: f64, t665: f64, t9157: f64, t2024: f64, t34976: f64, t9163: f64, t1971: f64, t2144: f64, t3351: f64, t41122: f64, t3148: f64, t3151: f64, t38350: f64) -> (f64, f64, f64, f64) {
    let t74092 = t16503 * t3369 * t665 * t9157;
    let t74096 = t16503 * t34976 * t2024 * t9163;
    let t74102 = t3351 * t1971 * t2144 * t41122;
    let t74105 = t38350 * t3148 * t3151;
    (t74092, t74096, t74102, t74105)
}
