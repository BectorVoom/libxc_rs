//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 799/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk799(t207: f64, t795: f64, t9580: f64, t2690: f64, t841: f64, t812: f64, t241: f64, t6589: f64, t67: f64, t815: f64, t836: f64, t1891: f64) -> (f64, f64, f64, f64, f64) {
    let t9583 = 0.16435185185185185185e-1_f64 * t9580 * t207 * t795;
    let t9600 = t841 * t2690;
    let t9601 = t812 * t9600;
    let t9607 = t241 * t6589 * t67;
    let t9637 = t815 * t836;
    let t9638 = t812 * t9637;
    let t9645 = t1891 * t67;
    (t9583, t9601, t9607, t9638, t9645)
}
