//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 304/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk304(t128: f64, t1614: f64, t326: f64, t1544: f64, t305: f64, t1547: f64, t118: f64, t1358: f64, t321: f64, t551: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1615 = t128 * t1614;
    let t1616 = t326 * t1615;
    let t1618 = t305 * t1544;
    let t1620 = t326 * t1547;
    let t1622 = t118 * t1358;
    let t1624 = t551 * t321;
    (t1615, t1616, t1618, t1620, t1622, t1624)
}
