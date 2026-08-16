//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 426/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk426(t121: f64, t1559: f64, t1: f64, t188: f64, t1628: f64, t200: f64) -> (f64, f64, f64, f64) {
    let t4538 = t121 * t1559;
    let t4539 = t4538 * t1;
    let t4540 = t188 * t4539;
    let t4614 = t1628 * t200;
    (t4538, t4539, t4540, t4614)
}
