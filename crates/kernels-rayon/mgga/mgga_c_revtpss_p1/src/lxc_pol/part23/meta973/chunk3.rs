//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3301/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3301(t1399: f64, t14193: f64, t14224: f64, t22005: f64, t22009: f64, t47444: f64, t5675: f64, t5745: f64, t5755: f64, t75269: f64, t75274: f64, t85580: f64, t86445: f64, t86506: f64, t86634: f64, t86639: f64, t86643: f64, t86647: f64) -> f64 {
    let t86649 = -0.32927245914677557992e-1_f64 * t75269 - 0.19756347548806534796e1_f64 * t5755 * t22009 * t14224 + 0.21951497276451705328e-1_f64 * t75274 + 0.92196288561097162379e1_f64 * t5745 * t86445 * t5675 - 0.19756347548806534796e1_f64 * t5755 * t86506 * t1399 - 0.11853808529283920877e2_f64 * t14193 * t22005 * t85580 - 0.32927245914677557992e-1_f64 * t86634 + 0.30356481678079769392e-1_f64 * t47444 - 0.9757440539382783019e-2_f64 * t86639 + 0.16463622957338778997e-1_f64 * t86643 - 0.32927245914677557992e-1_f64 * t86647;
    t86649
}
