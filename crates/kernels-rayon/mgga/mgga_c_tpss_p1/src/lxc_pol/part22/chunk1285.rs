//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1285/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1285(t1378: f64, t2364: f64, t2161: f64, t1639: f64, t3259: f64, t3326: f64, t10089: f64, t1625: f64, t3387: f64, t13111: f64, t3205: f64, t10444: f64, t116: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36075 = t1378 * t2364;
    let t36098 = t1378 * t2161;
    let t41371 = t1639 * t3259;
    let t41437 = t1639 * t3326;
    let t41590 = t1639 * t10089;
    let t41839 = t1625 * t3387;
    let t41867 = t13111 * t3205;
    let t41905 = t10444 * t116;
    (t36075, t36098, t41371, t41437, t41590, t41839, t41867, t41905)
}
