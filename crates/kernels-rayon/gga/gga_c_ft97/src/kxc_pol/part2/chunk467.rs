//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 467/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk467(t684: f64, t824: f64, t2665: f64, t446: f64, t2360: f64, t295: f64, t2349: f64, t666: f64, t89: f64, t1934: f64, t792: f64, t294: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2666 = t684 * t824;
    let t2667 = t2665 * t2666;
    let t2668 = t446 * t2667;
    let t2670 = t295 * t2360;
    let t2671 = t2670 * t2349;
    let t2673 = t89 * t666 * t2671;
    let t2675 = t792 * t1934;
    let t2677 = t89 * t666 * t2675;
    let t2679 = t797 * t294;
    (t2666, t2667, t2668, t2671, t2673, t2675, t2677, t2679)
}
