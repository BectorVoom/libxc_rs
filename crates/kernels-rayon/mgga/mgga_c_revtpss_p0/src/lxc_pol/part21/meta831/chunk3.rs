//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3103/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3103(t13148: f64, t56878: f64, t1121: f64, t3601: f64, t606: f64, t17728: f64, t460: f64, t489: f64, t17261: f64, t17373: f64, t12772: f64, t17639: f64, t3625: f64) -> (f64, f64, f64, f64, f64) {
    let t56997 = t13148 * t56878;
    let t56999 = t3601 * t1121 * t606;
    let t57005 = t460 * t489 * t17728;
    let t57021 = t17261 * t17373;
    let t57026 = t3625 * t12772 * t17639;
    (t56997, t56999, t57005, t57021, t57026)
}
