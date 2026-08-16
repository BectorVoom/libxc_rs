//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2116/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2116(t5: f64, t96409: f64, t96441: f64, t96478: f64, t96509: f64, t96545: f64, t96579: f64, t96605: f64, t96649: f64, t112: f64, t5456: f64, t6514: f64, t19534: f64, t88: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t96653 = piecewise3(t8, 0.0_f64, t96409 + t96441 + t96478 + t96509 + t96545 + t96579 + t96605 + t96649);
    let t96654 = t96653 * t112;
    let t96655 = t6514 * t5456;
    let t96657 = t88 * t19534;
    (t96654, t96655, t96657)
}
