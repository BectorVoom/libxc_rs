//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 409/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk409(t215: f64, t211: f64, t414: f64, t220: f64, t231: f64, t4: f64, t1220: f64, t283: f64, t482: f64, t132: f64, t762: f64, t737: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2013 = t215 * t215;
    let t2014 = 1.0_f64 / t2013;
    let t2018 = t211 * t414;
    let t2025 = t220 * t220;
    let t2026 = 1.0_f64 / t2025;
    let t2040 = t231 * t4;
    let t2042 = 0.10843580882781524214e-1_f64 * t2040 * t1220;
    let t2043 = t482 * t283;
    let t2046 = t132 * t762;
    let t2053 = t88 * t737;
    (t2014, t2018, t2026, t2042, t2043, t2046, t2053)
}
