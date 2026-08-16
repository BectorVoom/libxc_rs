//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 825/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk825(t5939: f64, t762: f64, t757: f64, t2079: f64, t754: f64, t46: f64, t752: f64) -> (f64, f64, f64, f64) {
    let t5940 = t5939 * t762;
    let t5941 = t757 * t5940;
    let t5943 = t2079 * t754;
    let t5944 = t5943 * t46;
    let t5945 = t752 * t5944;
    (t5940, t5941, t5943, t5945)
}
