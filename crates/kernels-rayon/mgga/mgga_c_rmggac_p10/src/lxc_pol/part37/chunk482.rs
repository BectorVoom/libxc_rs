//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 482/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk482(t13862: f64, t323: f64, t3133: f64, t3046: f64, t6444: f64, t333: f64, t3851: f64, t2048: f64, t793: f64, t328: f64, t3814: f64, t2566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13892 = t13862 * t323;
    let t13893 = t3133 * t13892;
    let t13895 = t6444 * t3046;
    let t13897 = t3046 * t333;
    let t13898 = t3851 * t13897;
    let t13900 = t793 * t2048;
    let t13902 = t3851 * t3046;
    let t13903 = t13902 * t328;
    let t13905 = t3814 * t3046;
    let t13906 = t13905 * t2566;
    (t13892, t13893, t13895, t13897, t13898, t13900, t13902, t13903, t13905, t13906)
}
