//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1331/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1331(t6238: f64, t7299: f64, t7284: f64, t24574: f64, t29546: f64, t225: f64, t29685: f64, t103345: f64, t2122: f64, t29674: f64, t29750: f64, t85853: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t103363 = t7299 * t6238;
    let t103391 = t7284 * t6238;
    let t103413 = t24574 * t29546;
    let t103464 = t29685 * t225;
    let t103490 = t2122 * t103345;
    let t103494 = t24574 * t29674;
    let t103507 = t85853 * t29750;
    (t103363, t103391, t103413, t103464, t103490, t103494, t103507)
}
