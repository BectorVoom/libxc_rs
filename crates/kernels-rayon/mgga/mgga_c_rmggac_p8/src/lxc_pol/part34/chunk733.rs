//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 733/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk733(t70818: f64, t14391: f64, t16156: f64, t68520: f64, t14639: f64, t2186: f64, t14563: f64, t2019: f64, t2020: f64, t270: f64, t702: f64, t31: f64, t7349: f64, t7351: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t70819 = 0.14905073231436680509e-2_f64 * t70818;
    let t70867 = t16156 * t14391;
    let t70877 = 0.29810146462873361016e-2_f64 * t68520;
    let t70885 = t2186 * t14639;
    let t70892 = t2019 * t2020 * t14563;
    let t70901 = t702 * t270;
    let t70904 = t7349 * t7351 * t70901 * t31;
    (t70819, t70867, t70877, t70885, t70892, t70901, t70904)
}
