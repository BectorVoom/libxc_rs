//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2439/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2439(t3147: f64, t698: f64, t973: f64, t10632: f64, t2924: f64, t10510: f64, t3114: f64, t10508: f64, t248: f64, t3039: f64, t3041: f64, t3020: f64, t3030: f64) -> (f64, f64, f64, f64, f64) {
    let t42613 = t973 * t698 * t3147;
    let t42671 = t10632 * t2924;
    let t42721 = t3114 * t10510;
    let t42735 = t3039 * t248 * t10508 * t3041;
    let t42741 = t3020 * t3030;
    (t42613, t42671, t42721, t42735, t42741)
}
