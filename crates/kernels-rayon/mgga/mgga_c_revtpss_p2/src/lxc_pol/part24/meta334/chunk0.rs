//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1165/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1165(t1544: f64, t5962: f64, t2477: f64, t828: f64, t23177: f64, t827: f64, t23245: f64, t18426: f64, t2747: f64, t6035: f64, t4364: f64, t4365: f64, t6017: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23279 = t1544 * t5962;
    let t23281 = t2477 * t828 * t23279;
    let t23285 = t827 * t828 * t23177;
    let t23289 = t827 * t828 * t23245;
    let t23293 = t2747 * t18426 * t6035;
    let t23297 = t4364 * t4365 * t6017;
    (t23279, t23281, t23285, t23289, t23293, t23297)
}
