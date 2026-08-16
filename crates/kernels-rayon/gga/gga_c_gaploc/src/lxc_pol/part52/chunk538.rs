//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 538/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk538(t10425: f64, t3407: f64, t7014: f64, t123: f64, t2754: f64, t883: f64) -> (f64, f64, f64, f64) {
    let t10426 = 0.14896037479937677779e-1_f64 * t10425;
    let t10427 = t7014 * t3407;
    let t10428 = 0.19171462976960374838e0_f64 * t10427;
    let t10429 = t2754 * t123;
    let t10430 = t10429 * t883;
    (t10426, t10427, t10428, t10430)
}
