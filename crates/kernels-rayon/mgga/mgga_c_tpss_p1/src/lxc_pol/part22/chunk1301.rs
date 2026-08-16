//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1301/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1301(t1981: f64, t234: f64, t38: f64, t5489: f64, t18646: f64, t5492: f64, t31450: f64, t5784: f64, t18338: f64, t5791: f64, t1985: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t62306 = t1981 * t38 * t234;
    let t62307 = t62306 * t5489;
    let t62309 = t5492 * t18646;
    let t62311 = t31450 * t5784;
    let t62314 = t18338 * t5791;
    let t62330 = t1981 * t1985 * t68;
    (t62306, t62307, t62309, t62311, t62314, t62330)
}
