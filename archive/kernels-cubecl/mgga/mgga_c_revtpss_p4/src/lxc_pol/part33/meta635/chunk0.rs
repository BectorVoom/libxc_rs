//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2083/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2083<F: Float>(t7058: F, t99201: F, t25375: F, t99349: F, t14983: F, t25399: F, t7064: F, t99321: F, t25411: F, t99389: F, t2435: F, t7774: F) -> (F, F, F, F, F, F) {
    let t99481 = t7058 * t99201;
    let t99485 = F::cast_from(0.28912093960683998208e-1_f64) * t25375 * t99349;
    let t99487 = F::cast_from(0.19514881078765566038e-1_f64) * t25399 * t14983;
    let t99491 = F::cast_from(0.25702851531048074406e-1_f64) * t7064 * t99321;
    let t99493 = F::cast_from(0.25702851531048074406e-1_f64) * t25411 * t99389;
    let t99495 = t7774 * t2435;
    (t99481, t99485, t99487, t99491, t99493, t99495)
}
