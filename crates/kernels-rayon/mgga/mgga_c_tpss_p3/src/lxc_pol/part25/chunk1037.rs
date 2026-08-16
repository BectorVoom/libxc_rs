//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1037/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1037(t14297: f64, t226: f64, t773: f64, t774: f64, t124: f64, t14029: f64, t762: f64, t2383: f64, t4771: f64, t801: f64, t4775: f64, t2143: f64, t4712: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14298 = t14297 * t226;
    let t14300 = t773 * t774 * t14298;
    let t14303 = t124 * t14029;
    let t14304 = t762 * t14303;
    let t14308 = t2383 * t4771;
    let t14311 = t801 * t774 * t14029;
    let t14314 = t2383 * t4775;
    let t14316 = t2143 * t4712;
    (t14298, t14300, t14304, t14308, t14311, t14314, t14316)
}
