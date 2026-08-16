//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 911/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk911(t2165: f64, t8292: f64, t2387: f64, t72: f64, t240: f64, t2116: f64, t226: f64, t339: f64, t769: f64, t790: f64, t2179: f64, t2133: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8293 = t8292 * t2165;
    let t8305 = t2387 * t72;
    let t8306 = t8305 * t240;
    let t8307 = t226 * t2116;
    let t8313 = t339 * t769 * t790;
    let t8314 = t8313 * t2179;
    let t8320 = t226 * t2133;
    (t8293, t8305, t8306, t8307, t8313, t8314, t8320)
}
