//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1274/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1274(t12268: f64, t3617: f64, t2258: f64, t3628: f64, t3367: f64, t471: f64, t2251: f64, t17350: f64, t3767: f64, t1121: f64, t1248: f64, t606: f64) -> (f64, f64, f64, f64, f64) {
    let t17550 = t3617 * t12268;
    let t17638 = t3628 * t2258;
    let t17643 = t471 * t3367;
    let t17644 = t17643 * t2251;
    let t17654 = t3767 * t17350;
    let t17655 = t1248 * t1121;
    let t17656 = t17655 * t606;
    (t17550, t17638, t17644, t17654, t17656)
}
