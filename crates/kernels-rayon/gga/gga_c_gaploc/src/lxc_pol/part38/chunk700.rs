//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 700/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk700(t11318: f64, t874: f64, t1445: f64, t574: f64, t12954: f64, t12958: f64, t13276: f64, t1457: f64, t4540: f64, t2854: f64, t3338: f64, t11413: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13420 = t11318 * t874;
    let t13421 = t1445 * t13420;
    let t13423 = 0.92023022289409799224e1_f64 * t574 * t13421;
    let t13424 = 0.23005755572352449806e1_f64 * t12954;
    let t13425 = 0.15337170381568299871e1_f64 * t12958;
    let t13426 = t1457 * t13276;
    let t13428 = 0.21450293971110256001e1_f64 * t4540 * t13426;
    let t13429 = t2854 * t3338;
    let t13430 = t1445 * t13429;
    let t13433 = t11413 * t874;
    (t13420, t13421, t13423, t13424, t13425, t13426, t13428, t13429, t13430, t13433)
}
