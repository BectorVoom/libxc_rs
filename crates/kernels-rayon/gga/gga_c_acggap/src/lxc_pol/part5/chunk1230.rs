//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1230/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1230(t1745: f64, t974: f64, t4389: f64, t6332: f64, t1886: f64, t3237: f64, t6110: f64, t997: f64, t1036: f64, t1037: f64, t386: f64, t5679: f64) -> (f64, f64, f64, f64, f64) {
    let t22538 = t974 * t1745;
    let t22540 = t4389 * t6332;
    let t22544 = t3237 * t1886;
    let t22546 = t997 * t6110;
    let t22550 = t1036 * t386 * t5679 * t1037;
    (t22538, t22540, t22544, t22546, t22550)
}
