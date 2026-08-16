//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 742/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk742(t5939: f64, t762: f64, t757: f64, t749: f64, t5717: f64) -> (f64, f64, f64, f64) {
    let t5940 = t5939 * t762;
    let t5941 = t757 * t5940;
    let t5950 = t749 * t749;
    let t5951 = 1.0_f64 / t5950;
    let t5952 = t5717 * t5951;
    (t5941, t5950, t5951, t5952)
}
