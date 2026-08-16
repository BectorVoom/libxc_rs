//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 733/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk733(t14639: f64, t2186: f64, t14563: f64, t2019: f64, t2020: f64, t270: f64, t702: f64, t31: f64, t7349: f64, t7351: f64, t14683: f64, t7244: f64) -> (f64, f64, f64, f64, f64) {
    let t70885 = t2186 * t14639;
    let t70892 = t2019 * t2020 * t14563;
    let t70901 = t702 * t270;
    let t70904 = t7349 * t7351 * t70901 * t31;
    let t70905 = 0.43368970657079495312e-4_f64 * t70904;
    let t70929 = t7244 * t14683;
    (t70885, t70892, t70901, t70905, t70929)
}
