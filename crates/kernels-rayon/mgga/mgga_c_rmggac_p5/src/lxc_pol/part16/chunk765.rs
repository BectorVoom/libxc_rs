//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 765/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk765(t2078: f64, t3851: f64, t7834: f64, t797: f64, t128: f64, t305: f64, t3899: f64, t321: f64, t830: f64, t262: f64, t2068: f64, t2067: f64, t25529: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35815 = t3851 * t2078;
    let t35824 = t797 * t7834;
    let t35861 = t305 * t128 * t3899;
    let t35875 = t830 * t321;
    let t35876 = t262 * t35875;
    let t35877 = t2068 * t35876;
    let t35879 = t25529 * t2067;
    (t35815, t35824, t35861, t35875, t35876, t35877, t35879)
}
