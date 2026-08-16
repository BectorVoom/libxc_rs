//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 906/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk906(t31528: f64, t31729: f64, t31751: f64, t31779: f64, t3: f64, t112: f64, t8646: f64, t1873: f64, t24462: f64, t24465: f64, t7015: f64, t6534: f64, t7230: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31781 = t31528 + t31729 + t31751 + t31779;
    let t31782 = t3 * t31781;
    let t31795 = t8646 * t112;
    let t31799 = 0.135e2_f64 * t24462 * t1873;
    let t31801 = 27.0_f64 * t24465 * t7015;
    let t31803 = 0.135e2_f64 * t7230 * t6534;
    (t31781, t31782, t31795, t31799, t31801, t31803)
}
