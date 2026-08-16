//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 282/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk282(t570: f64, t874: f64, t352: f64, t321: f64, t559: f64, t235: f64, t837: f64) -> (f64, f64, f64, f64) {
    let t1357 = t874 * t570;
    let t1358 = t1357 * t352;
    let t1361 = t559 * t321;
    let t1364 = t235 * t837;
    (t1357, t1358, t1361, t1364)
}
