//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1355/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1355(t473: f64, t6695: f64, t1214: f64, t3759: f64, t6587: f64, t1280: f64, t21082: f64, t21471: f64, t5284: f64, t5332: f64, t1269: f64, t1287: f64, t6622: f64) -> (f64, f64, f64, f64, f64) {
    let t21541 = t473 * t6695;
    let t21542 = t21541 * t1214;
    let t21551 = t3759 * t6587;
    let t21554 = t1280 * t21082;
    let t21557 = t21471 * t5284;
    let t21558 = t5332 * t21557;
    let t21562 = t1269 * t6622 * t1287;
    (t21542, t21551, t21554, t21558, t21562)
}
