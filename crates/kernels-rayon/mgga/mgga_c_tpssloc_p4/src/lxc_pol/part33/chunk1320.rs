//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1320/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1320(t105308: f64, t105350: f64, t105383: f64, t105417: f64, t6552: f64, t7479: f64, t98133: f64, t1880: f64, t21013: f64, t214: f64, t225: f64, t258: f64) -> (f64, f64, f64) {
    let t105419 = t105308 + t105350 + t105383 + t105417;
    let t105423 = t6552 * t98133 * t7479;
    let t105428 = t1880 * t214 * t21013 * t225 * t258;
    (t105419, t105423, t105428)
}
