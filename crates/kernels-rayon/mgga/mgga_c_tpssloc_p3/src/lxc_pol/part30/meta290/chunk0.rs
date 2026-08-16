//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1295/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1295(t207: f64, t795: f64, t9580: f64, t2690: f64, t841: f64, t812: f64, t849: f64, t241: f64, t6589: f64, t67: f64, t2632: f64, t776: f64) -> (f64, f64, f64, f64, f64) {
    let t9583 = 0.16435185185185185185e-1_f64 * t9580 * t207 * t795;
    let t9600 = t841 * t2690;
    let t9601 = t812 * t9600;
    let t9602 = t9601 * t849;
    let t9607 = t241 * t6589 * t67;
    let t9627 = t2632 * t776;
    (t9583, t9601, t9602, t9607, t9627)
}
