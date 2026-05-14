//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1321/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1321<F: Float>(t1444: F, t6895: F, t9657: F, t22307: F, t225: F, t212: F, t6888: F, t1358: F, t689: F, t1357: F, t6896: F, t72: F, t686: F, t9680: F, t10160: F, t10163: F, t10166: F, t1424: F, t14280: F, t14290: F, t14294: F, t14297: F, t213: F, t4071: F, t561: F, t6919: F) -> (F,) {
    let t22432 = t6895 * t1444;
    let t22433 = t9657 * t22432;
    let t22441 = t22307 * t225;
    let t22445 = t212 * t6888;
    let t22446 = t22445 * t1358;
    let t22447 = t689 * t22446;
    let t22449 = t1357 * t6896;
    let t22450 = t689 * t22449;
    let t22452 = t6895 * t72;
    let t22453 = t22452 * t686;
    let t22454 = t9680 * t22453;
    let t22459 = -0.26019841438354088051e-1 * t14280 - 0.39512695097613069591e1 * t1424 * t22433 - 0.65854491829355115987e0 * t4071 * t6919 - 0.73171657588172351096e-2 * t10160 + 0.65049603595885220126e-3 * t10163 + 0.11565819519348392139e-2 * t10166 + 0.65854491829355115987e0 * t213 * t22441 * t561 - 0.54878743191129263322e-2 * t22447 - 0.10975748638225852664e-1 * t22450 + 0.19514881078765566037e-1 * t22454 - 0.14634331517634470219e-1 * t14290 + 0.23131639038696784278e-2 * t14294 + 0.13009920719177044025e-2 * t14297;
    (t22459,)
}
