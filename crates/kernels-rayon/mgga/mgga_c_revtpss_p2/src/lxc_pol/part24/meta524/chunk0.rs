//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1555/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1555(t21272: f64, t5378: f64, t12772: f64, t24793: f64, t3625: f64, t24803: f64, t44425: f64, t1208: f64, t24697: f64, t225: f64, t480: f64, t17438: f64, t20846: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t83018 = t21272 * t5378;
    let t83047 = t3625 * t12772 * t24793;
    let t83067 = t3625 * t44425 * t24803;
    let t83107 = t24697 * t1208;
    let t83108 = t83107 * t225;
    let t83109 = t83108 * t480;
    let t83112 = t17438 * t20846;
    (t83018, t83047, t83067, t83107, t83108, t83109, t83112)
}
