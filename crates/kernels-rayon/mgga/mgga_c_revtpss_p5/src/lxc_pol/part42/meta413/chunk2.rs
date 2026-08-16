//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1459/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1459(t22452: f64, t686: f64, t9680: f64, t10160: f64, t10163: f64, t10166: f64, t1424: f64, t14280: f64, t14290: f64, t14294: f64, t14297: f64, t213: f64, t22433: f64, t22441: f64, t22447: f64, t22450: f64, t4071: f64, t561: f64, t6919: f64) -> f64 {
    let t22453 = t22452 * t686;
    let t22454 = t9680 * t22453;
    let t22459 = -0.26019841438354088051e-1_f64 * t14280 - 0.39512695097613069591e1_f64 * t1424 * t22433 - 0.65854491829355115987e0_f64 * t4071 * t6919 - 0.73171657588172351096e-2_f64 * t10160 + 0.65049603595885220126e-3_f64 * t10163 + 0.11565819519348392139e-2_f64 * t10166 + 0.65854491829355115987e0_f64 * t213 * t22441 * t561 - 0.54878743191129263322e-2_f64 * t22447 - 0.10975748638225852664e-1_f64 * t22450 + 0.19514881078765566037e-1_f64 * t22454 - 0.14634331517634470219e-1_f64 * t14290 + 0.23131639038696784278e-2_f64 * t14294 + 0.13009920719177044025e-2_f64 * t14297;
    t22459
}
