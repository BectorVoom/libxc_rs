//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 888/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk888(t33398: f64, t858: f64, t26728: f64, t7516: f64, t6571: f64, t7841: f64, t6553: f64, t1880: f64, t1492: f64, t8543: f64, t218: f64, t33395: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33399 = t858 * t33398;
    let t33405 = t26728 * t7516;
    let t33408 = t6571 * t7841;
    let t33409 = t6553 * t33408;
    let t33410 = t1880 * t33409;
    let t33412 = t1492 * t8543;
    let t33414 = t218 * t33395;
    (t33399, t33405, t33408, t33409, t33410, t33412, t33414)
}
