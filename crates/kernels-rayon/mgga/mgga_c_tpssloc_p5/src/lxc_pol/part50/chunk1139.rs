//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1139/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1139(t112943: f64, t6562: f64, t6572: f64, t234: f64, t6624: f64, t22893: f64, t23164: f64, t30677: f64, t23168: f64, t30678: f64, t30686: f64, t6579: f64) -> (f64, f64, f64, f64, f64) {
    let t112948 = t6562 * t112943 * t6572;
    let t112951 = t234 * t6624;
    let t112961 = t23164 * t22893 * t30677;
    let t112968 = t23168 * t30678;
    let t112974 = t6579 * t30686;
    (t112948, t112951, t112961, t112968, t112974)
}
