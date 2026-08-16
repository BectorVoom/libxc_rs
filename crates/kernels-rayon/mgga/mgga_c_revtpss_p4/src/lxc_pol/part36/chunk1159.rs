//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1159/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1159(t5362: f64, t7613: f64, t1256: f64, t8177: f64, t8185: f64, t2137: f64, t5389: f64, t467: f64, t2138: f64, t5326: f64, t800: f64, t8171: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29065 = t7613 * t5362;
    let t29072 = t8177 * t1256;
    let t29077 = t8185 * t1256;
    let t29082 = t2137 * t5389;
    let t29083 = t467 * t29082;
    let t29086 = t5326 * t2138;
    let t29089 = t8171 * t800;
    (t29065, t29072, t29077, t29082, t29083, t29086, t29089)
}
