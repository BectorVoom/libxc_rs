//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1007/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1007(t1173: f64, t674: f64, t9824: f64, t1997: f64, t2320: f64, t38967: f64, t1971: f64, t333: f64, t7230: f64, t880: f64, t9969: f64, t2144: f64, t352: f64) -> (f64, f64, f64, f64) {
    let t47029 = t9824 * t1173 * t674;
    let t47030 = t47029 * t1997;
    let t47032 = t38967 * t2320;
    let t47037 = t7230 * t1971 * t880 * t9969 * t333;
    let t47042 = t7230 * t1971 * t2144 * t9969 * t352;
    (t47030, t47032, t47037, t47042)
}
