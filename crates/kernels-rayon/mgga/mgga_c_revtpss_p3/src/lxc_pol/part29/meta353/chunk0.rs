//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1282/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1282(t1178: f64, t3519: f64, t439: f64, t3522: f64, t447: f64, t300: f64, t3488: f64, t3800: f64, t498: f64, t1204: f64, t1269: f64, t12295: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12552 = 1.0_f64 / t3519 / t1178;
    let t12553 = t439 * t12552;
    let t12555 = 1.0_f64 / t3522 / t447;
    let t12571 = t300 * t3488;
    let t12587 = 1.0_f64 / t3800 / t498;
    let t12603 = t1204 * t1269;
    let t12610 = 0.46096296296296296297e-1_f64 * t12295;
    (t12552, t12553, t12555, t12571, t12587, t12603, t12610)
}
