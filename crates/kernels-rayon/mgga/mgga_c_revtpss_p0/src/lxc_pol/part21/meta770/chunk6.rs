//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2732/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2732(t2439: f64, t4469: f64, t780: f64, t785: f64, t213: f64, t252: f64, t2440: f64, t4534: f64, t1580: f64, t41117: f64, t10494: f64, t15011: f64, t2829: f64, t40982: f64, t40986: f64, t40988: f64, t40994: f64, t40998: f64, t50161: f64, t50219: f64, t50221: f64, t50223: f64, t50227: f64, t50232: f64) -> f64 {
    let t50236 = t2439 * t785 * t4469 * t780;
    let t50240 = t213 * t252;
    let t50245 = t2439 * t2440 * t4534;
    let t50248 = t41117 * t1580;
    let t50250 = -t50219 - t50221 - t50223 - 0.19756347548806534796e1_f64 * t15011 * t2829 - 0.17563392970889009433e0_f64 * t50227 - 0.54878743191129263322e-2_f64 * t40982 - 0.16463622957338778996e-1_f64 * t50232 - 0.19514881078765566037e-2_f64 * t50236 - 0.21951497276451705329e-1_f64 * t40986 - 0.51220160311720645767e-1_f64 * t40988 - 0.11853808529283920877e2_f64 * t50240 * t50161 * t10494 + 0.19514881078765566037e-2_f64 * t50245 + 0.21951497276451705329e-1_f64 * t40994 + 0.11044544084478153697e-3_f64 * t50248 - t40998;
    t50250
}
