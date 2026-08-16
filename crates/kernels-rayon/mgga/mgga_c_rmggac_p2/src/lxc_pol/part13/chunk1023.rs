//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1023/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1023(t8529: f64, t10285: f64, t10286: f64, t37108: f64, t42424: f64, t42425: f64, t42426: f64, t42427: f64, t42428: f64, t7383: f64, t7391: f64, t9333: f64) -> (f64, f64) {
    let t42429 = 0.5454932330849068346e-1_f64 * t8529;
    let t42431 = 0.31931311204970156171e0_f64 * t7383 - t42424 + t42425 + t42426 + t42427 + t42428 - t42429 + t10285 - t10286 - t37108 + 0.17347588262831798123e-3_f64 * t7391;
    let t42434 = 0.11974241701863808564e0_f64 * t9333;
    (t42431, t42434)
}
