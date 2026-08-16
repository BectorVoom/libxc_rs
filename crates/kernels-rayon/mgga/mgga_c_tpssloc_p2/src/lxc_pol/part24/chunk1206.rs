//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1206/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1206(t6688: f64, t974: f64, t23631: f64, t381: f64, t883: f64, t6743: f64, t14227: f64, t6800: f64, t23384: f64, t6790: f64, t1949: f64, t3010: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23632 = t974 * t6688;
    let t23633 = t23631 * t23632;
    let t23634 = t381 * t883;
    let t23635 = t6743 * t23634;
    let t23636 = t14227 * t6800;
    let t23637 = t23635 * t23636;
    let t23642 = t23384 * t6790;
    let t23644 = t3010 * t1949;
    (t23632, t23633, t23634, t23635, t23636, t23637, t23642, t23644)
}
