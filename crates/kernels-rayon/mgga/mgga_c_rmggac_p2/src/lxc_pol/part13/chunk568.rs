//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 568/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk568(t2060: f64, t4905: f64, t903: f64, t446: f64, t457: f64, t201: f64, t1979: f64, t1982: f64, t1162: f64, t194: f64, t1320: f64, t1322: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7524 = t2060 * t4905;
    let t7525 = t903 * t7524;
    let t7541 = t446 * t457;
    let t7542 = t7541 * t201;
    let t7544 = t7542 * t1979 * t1982;
    let t7546 = t194 * t1162;
    let t7547 = t7546 * t201;
    let t7549 = t7547 * t1979 * t1982;
    let t7551 = t1320 * t1322;
    (t7524, t7525, t7541, t7542, t7544, t7546, t7547, t7549, t7551)
}
