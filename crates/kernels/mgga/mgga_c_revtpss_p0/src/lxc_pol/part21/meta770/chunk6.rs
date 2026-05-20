//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2732/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2732<F: Float>(t2439: F, t4469: F, t780: F, t785: F, t213: F, t252: F, t2440: F, t4534: F, t1580: F, t41117: F, t10494: F, t15011: F, t2829: F, t40982: F, t40986: F, t40988: F, t40994: F, t40998: F, t50161: F, t50219: F, t50221: F, t50223: F, t50227: F, t50232: F) -> F {
    let t50236 = t2439 * t785 * t4469 * t780;
    let t50240 = t213 * t252;
    let t50245 = t2439 * t2440 * t4534;
    let t50248 = t41117 * t1580;
    let t50250 = -t50219 - t50221 - t50223 - F::cast_from(0.19756347548806534796e1_f64) * t15011 * t2829 - F::cast_from(0.17563392970889009433e0_f64) * t50227 - F::cast_from(0.54878743191129263322e-2_f64) * t40982 - F::cast_from(0.16463622957338778996e-1_f64) * t50232 - F::cast_from(0.19514881078765566037e-2_f64) * t50236 - F::cast_from(0.21951497276451705329e-1_f64) * t40986 - F::cast_from(0.51220160311720645767e-1_f64) * t40988 - F::cast_from(0.11853808529283920877e2_f64) * t50240 * t50161 * t10494 + F::cast_from(0.19514881078765566037e-2_f64) * t50245 + F::cast_from(0.21951497276451705329e-1_f64) * t40994 + F::cast_from(0.11044544084478153697e-3_f64) * t50248 - t40998;
    t50250
}
