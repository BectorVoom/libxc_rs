//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2692/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2692(t2782: f64, t4077: f64, t47794: f64, t556: f64, t1426: f64, t5711: f64, t786: f64, t3917: f64, t1424: f64, t14269: f64, t14299: f64, t213: f64, t225: f64, t4071: f64, t4132: f64, t47568: f64, t47570: f64, t47574: f64, t47580: f64, t47591: f64, t49161: f64, t49468: f64, t49472: f64, t49474: f64, t49477: f64, t49480: f64, t561: f64, t5774: f64, t9657: f64) -> f64 {
    let t49497 = t2782 * t556 * t47794 * t4077;
    let t49503 = t786 * t5711 * t1426;
    let t49504 = t49503 * t3917;
    let t49506 = -0.17073386770573548589e-1_f64 * t49468 + 0.58544643236296698113e-1_f64 * t49472 - 0.11044544084478153697e-3_f64 * t49474 - t49477 - 0.19514881078765566037e-2_f64 * t49480 - 0.19756347548806534796e1_f64 * t4071 * t14269 + 0.33133632253434461091e-3_f64 * t47568 - 0.11853808529283920877e2_f64 * t1424 * t9657 * t5774 * t4077 - 0.13878983423218070567e-1_f64 * t47570 + 0.65854491829355115987e0_f64 * t213 * t49161 * t225 * t561 - 0.58911598146606471822e-3_f64 * t47574 + 0.98781737744032673976e-1_f64 * t49497 - 0.78059524315062264151e-1_f64 * t47580 - 0.19756347548806534796e1_f64 * t14299 * t4132 - 0.58544643236296698113e-1_f64 * t49504 - t47591;
    t49506
}
