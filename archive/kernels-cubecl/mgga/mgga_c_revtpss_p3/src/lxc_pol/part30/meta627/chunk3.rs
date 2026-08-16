//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2177/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2177<F: Float>(t25375: F, t99161: F, t1580: F, t25338: F, t689: F, t25365: F, t27279: F, t7058: F, t99201: F, t99349: F, t14983: F, t25399: F) -> (F, F, F, F, F, F) {
    let t99472 = F::cast_from(0.28912093960683998208e-1_f64) * t25375 * t99161;
    let t99475 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t25338 * t1580;
    let t99480 = F::cast_from(0.25702851531048074406e-1_f64) * t25365 * t27279;
    let t99481 = t7058 * t99201;
    let t99485 = F::cast_from(0.28912093960683998208e-1_f64) * t25375 * t99349;
    let t99487 = F::cast_from(0.19514881078765566038e-1_f64) * t25399 * t14983;
    (t99472, t99475, t99480, t99481, t99485, t99487)
}
