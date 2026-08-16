//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1511/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1511(t4343: f64, t854: f64, t236: f64, t807: f64, t124: f64, t14468: f64, t800: f64, t775: f64) -> (f64, f64, f64, f64) {
    let t14741 = t854 * t4343;
    let t14742 = t236 * t14741;
    let t14744 = 0.57165357490759649296e-4_f64 * t807 * t14742;
    let t14745 = t124 * t14468;
    let t14746 = t800 * t14745;
    let t14749 = t4343 * t775;
    (t14741, t14744, t14746, t14749)
}
