//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1316/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1316(t20853: f64, t6605: f64, t815: f64, t20944: f64, t81959: f64, t1894: f64, t20756: f64, t236: f64, t81969: f64, t20994: f64, t6581: f64, t20800: f64, t6591: f64) -> (f64, f64, f64, f64, f64) {
    let t105353 = t6605 * t815 * t20853;
    let t105366 = t81959 * t20944;
    let t105370 = t81969 * t1894 * t236 * t20756;
    let t105372 = t6581 * t20994;
    let t105376 = t6591 * t1894 * t236 * t20800;
    (t105353, t105366, t105370, t105372, t105376)
}
