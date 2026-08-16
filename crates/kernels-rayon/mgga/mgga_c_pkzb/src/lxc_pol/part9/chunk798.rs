//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 798/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk798(t2038: f64, t5708: f64, t2023: f64, t768: f64, t46: f64, t2037: f64, t747: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5709 = t2038 * t5708;
    let t5711 = t768 * t2023;
    let t5712 = t5711 * t46;
    let t5713 = t2037 * t5712;
    let t5716 = t747 * t747;
    let t5717 = 1.0_f64 / t5716;
    (t5709, t5711, t5712, t5713, t5716, t5717)
}
