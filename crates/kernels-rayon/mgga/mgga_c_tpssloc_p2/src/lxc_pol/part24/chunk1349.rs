//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1349/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1349(t10103: f64, t1880: f64, t6553: f64, t6571: f64, t6552: f64, t6554: f64, t9516: f64, t23164: f64, t23204: f64, t23222: f64, t23168: f64, t23238: f64) -> (f64, f64, f64, f64) {
    let t82165 = t1880 * t6553 * t6571 * t10103;
    let t82169 = t6552 * t6553 * t6554 * t9516;
    let t82172 = t23164 * t23204 * t23222;
    let t82174 = t23168 * t23238;
    (t82165, t82169, t82172, t82174)
}
