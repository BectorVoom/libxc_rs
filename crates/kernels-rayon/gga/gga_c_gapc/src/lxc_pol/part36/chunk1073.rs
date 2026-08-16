//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1073/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1073(t772: f64, t9786: f64, t9787: f64, t11948: f64, t29350: f64, t10039: f64, t3438: f64, t11479: f64, t2767: f64, t7294: f64, t11748: f64, t2594: f64) -> (f64, f64, f64, f64, f64) {
    let t33185 = t9786 * t772 * t9787;
    let t33187 = t11948 * t29350;
    let t33190 = t3438 * t772 * t10039;
    let t33193 = t7294 * t11479 * t2767;
    let t33195 = t11748 * t2594;
    (t33185, t33187, t33190, t33193, t33195)
}
