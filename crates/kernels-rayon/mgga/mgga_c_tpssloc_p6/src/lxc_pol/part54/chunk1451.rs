//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1451/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1451(t31817: f64, t33185: f64, t1873: f64, t94127: f64, t120849: f64, t8657: f64, t75795: f64, t100993: f64, t7769: f64, t24465: f64, t26542: f64, t26545: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t122788 = 27.0_f64 * t33185 * t31817;
    let t122790 = 0.135e2_f64 * t94127 * t1873;
    let t122794 = 27.0_f64 * t120849 * t8657;
    let t122800 = 27.0_f64 * t75795 * t8657;
    let t122804 = 27.0_f64 * t100993 * t7769;
    let t122806 = 27.0_f64 * t24465 * t26542;
    let t122808 = 27.0_f64 * t24465 * t26545;
    (t122788, t122790, t122794, t122800, t122804, t122806, t122808)
}
