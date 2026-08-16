//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 334/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk334(t1101: f64, t581: f64, t926: f64, t451: f64, t453: f64) -> (f64, f64, f64, f64, f64) {
    let t1102 = t1101 * t581;
    let t1103 = t926 * t1102;
    let t1106 = t451 * t451;
    let t1107 = 1.0_f64 / t1106;
    let t1108 = t1107 * t453;
    (t1102, t1103, t1106, t1107, t1108)
}
