//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 946/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk946(t10421: f64, t7030: f64, t544: f64, t8237: f64, t9287: f64, t3407: f64, t7014: f64, t123: f64, t2754: f64, t883: f64, t2488: f64, t2487: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10422 = t10421 * t7030;
    let t10423 = 0.14896037479937677779e-1_f64 * t10422;
    let t10424 = t544 * t8237;
    let t10425 = t10424 * t9287;
    let t10426 = 0.14896037479937677779e-1_f64 * t10425;
    let t10427 = t7014 * t3407;
    let t10428 = 0.19171462976960374838e0_f64 * t10427;
    let t10429 = t2754 * t123;
    let t10430 = t10429 * t883;
    let t10431 = t2488 * t10430;
    let t10432 = t2487 * t10431;
    (t10423, t10424, t10426, t10428, t10429, t10430, t10431, t10432)
}
