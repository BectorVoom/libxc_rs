//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1152/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1152(t10244: f64, t2380: f64, t6475: f64, t3214: f64, t8363: f64, t204: f64, t648: f64, t9795: f64) -> (f64, f64, f64) {
    let t27232 = t2380 * t6475 * t10244;
    let t27234 = t3214 * t8363;
    let t27262 = t204 * t648 * t9795;
    (t27232, t27234, t27262)
}
