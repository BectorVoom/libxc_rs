//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 897/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk897(t15049: f64, t2604: f64, t15128: f64, t352: f64, t262: f64, t8620: f64, t1971: f64, t3351: f64, t7190: f64, t8950: f64, t7262: f64, t8979: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76075 = 0.2993560425465952141e-1_f64 * t2604 * t15049;
    let t76077 = t15128 * t352;
    let t76078 = t262 * t76077;
    let t76079 = t8620 * t76078;
    let t76084 = 0.10215503974391481456e-3_f64 * t3351 * t1971 * t7190 * t8950;
    let t76087 = t3351 * t1971 * t7262 * t8979;
    (t76075, t76077, t76078, t76079, t76084, t76087)
}
