//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 905/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk905(t10151: f64, t2464: f64, t2465: f64, t2487: f64, t10417: f64, t1415: f64, t7030: f64, t12960: f64, t31051: f64, t10473: f64, t2478: f64, t6576: f64) -> (f64, f64, f64, f64) {
    let t41640 = t2487 * t2464 * t2465 * t10151;
    let t41643 = t1415 * t10417 * t7030;
    let t41645 = t31051 * t12960;
    let t41646 = 0.19171462976960374838e1_f64 * t41645;
    let t41649 = t6576 * t10473 * t2478;
    (t41640, t41643, t41646, t41649)
}
