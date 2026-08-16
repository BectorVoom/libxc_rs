//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 987/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk987(t11109: f64, t7810: f64, t10628: f64, t4820: f64, t7513: f64, t1029: f64, t2617: f64, t7803: f64, t1052: f64, t7822: f64, t2972: f64, t7324: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11110 = t7810 * t11109;
    let t11111 = 0.19171462976960374838e0_f64 * t11110;
    let t11116 = t4820 * t10628;
    let t11118 = 0.79445533226334281487e-1_f64 * t7513 * t11116;
    let t11119 = t1029 * t2617;
    let t11120 = t7803 * t11119;
    let t11121 = 0.19171462976960374838e0_f64 * t11120;
    let t11130 = t7822 * t1052;
    let t11132 = 2.0_f64 * t7324 * t2972;
    (t11111, t11116, t11118, t11119, t11121, t11130, t11132)
}
