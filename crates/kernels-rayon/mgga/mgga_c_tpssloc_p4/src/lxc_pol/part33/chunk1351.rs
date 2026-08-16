//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1351/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1351(t1935: f64, t1941: f64, t21456: f64, t21482: f64, t21546: f64, t25645: f64, t28566: f64, t28578: f64, t28587: f64, t343: f64, t378: f64, t6717: f64, t6734: f64, t7574: f64, t7583: f64, t83080: f64, t88372: f64, t99662: f64, t99667: f64, t99671: f64, t99707: f64) -> f64 {
    let t106307 = 7.0_f64 / 648.0_f64 * t6717 * t21546 + t99707 / 1152.0_f64 - 0.30279567070605293142e-3_f64 * t7574 * t28566 - 0.10093189023535097714e-3_f64 * t1935 * t21456 * t343 * t6734 + t21482 * t1941 * t378 / 1536.0_f64 - 0.60559134141210586284e-3_f64 * t99662 * t7583 - 0.30279567070605293142e-3_f64 * t99667 * t7583 - 0.30279567070605293142e-3_f64 * t25645 * t28587 - 0.60559134141210586284e-3_f64 * t88372 * t28578 + t83080 - 0.30279567070605293142e-3_f64 * t99671 * t7583;
    t106307
}
