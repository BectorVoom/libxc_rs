//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 873/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk873(t15625: f64, t632: f64, t72: f64, t4872: f64, t8618: f64, t637: f64, t643: f64, t4861: f64, t8675: f64, t358: f64, t4883: f64, t363: f64) -> (f64, f64, f64, f64) {
    let t17564 = t72 * t632 * t15625;
    let t17567 = t8618 * t4872;
    let t17569 = t637 * t17567 * t643;
    let t17573 = t8675 * t4861;
    let t17575 = t4883 * t358;
    let t17576 = t17575 * t363;
    (t17564, t17569, t17573, t17576)
}
