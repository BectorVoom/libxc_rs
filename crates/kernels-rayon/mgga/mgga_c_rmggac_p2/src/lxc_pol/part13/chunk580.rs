//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 580/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk580(t36: f64, t5245: f64, t344: f64, t830: f64, t3839: f64, t7634: f64, t1243: f64, t128: f64, t118: f64, t2001: f64, t675: f64, t1987: f64, t2191: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7660 = t5245 * t36;
    let t7662 = t344 * t830;
    let t7664 = t3839 * t7634;
    let t7675 = t128 * t1243;
    let t7676 = t118 * t7675;
    let t7677 = t2001 * t7676;
    let t7678 = t675 * t7677;
    let t7680 = t2191 * t1987;
    (t7660, t7662, t7664, t7677, t7678, t7680)
}
