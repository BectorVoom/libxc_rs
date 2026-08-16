//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 549/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk549(t14425: f64, t13957: f64, t8041: f64, t1356: f64, t14276: f64, t14278: f64, t14280: f64, t2228: f64, t36: f64, t305: f64, t664: f64, t8264: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14426 = 0.11974241701863808564e0_f64 * t14425;
    let t14427 = t8041 * t13957;
    let t14428 = t1356 * t14427;
    let t14429 = 0.11974241701863808564e0_f64 * t14428;
    let t14431 = 0.20455996240684006298e-1_f64 * t14276;
    let t14432 = 0.2727466165424534173e-1_f64 * t14278;
    let t14433 = 0.13637330827122670865e-1_f64 * t14280;
    let t14438 = t2228 * t36;
    let t14439 = t305 * t14438;
    let t14440 = 0.14967802127329760705e-1_f64 * t14439;
    let t14441 = t8264 * t664;
    (t14426, t14427, t14429, t14431, t14432, t14433, t14438, t14440, t14441)
}
