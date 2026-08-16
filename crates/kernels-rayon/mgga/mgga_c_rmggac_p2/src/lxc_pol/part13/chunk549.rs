//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 549/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk549(t31: f64, t357: f64, t2046: f64, t2050: f64, t1990: f64, t2186: f64, t1271: f64, t1986: f64, t675: f64, t4443: f64, t671: f64, t674: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7393 = t357 * t31;
    let t7395 = t2046 * t2050 * t7393;
    let t7402 = t2186 * t1990;
    let t7404 = t1986 * t1271;
    let t7405 = t675 * t7404;
    let t7407 = t671 * t4443;
    let t7408 = t7407 * t674;
    (t7393, t7395, t7402, t7404, t7405, t7407, t7408)
}
