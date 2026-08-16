//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1305/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1305(t3382: f64, t5807: f64, t1077: f64, t1180: f64, t1181: f64, t127: f64, t129: f64, t14405: f64, t14414: f64, t14419: f64, t14421: f64, t14429: f64, t145: f64, t1552: f64, t1759: f64, t18817: f64, t18819: f64, t18828: f64, t18830: f64, t22607: f64, t5: f64) -> f64 {
    let t24294 = t3382 * t5807;
    let t24296 = 0.17149607247227894789e-2_f64 * t1180 * t1181 * t1552 * t1759 * t1077 - 0.85748036236139473944e-3_f64 * t18817 - 0.32012600194825403606e-1_f64 * t18819 - t14405 - t14414 - t14419 - t14421 + 0.1133779590233399711e0_f64 * t14429 - 7.0_f64 / 144.0_f64 * t18828 + t127 * t129 * t5 * t22607 * t145 / 96.0_f64 - 455.0_f64 / 648.0_f64 * t18830 + 0.17149607247227894789e-2_f64 * t24294;
    t24296
}
