//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 829/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk829(t15254: f64, t16043: f64, t2123: f64, t3351: f64, t515: f64, t618: f64, t7231: f64, t1528: f64, t664: f64, t15258: f64, t3352: f64, t41059: f64) -> (f64, f64, f64, f64, f64) {
    let t74891 = t16043 * t15254;
    let t74896 = t3351 * t7231 * t515 * t2123 * t618;
    let t74901 = t3351 * t7231 * t515 * t664 * t1528;
    let t74903 = t16043 * t15258;
    let t74909 = t3351 * t3352 * t515 * t41059;
    (t74891, t74896, t74901, t74903, t74909)
}
