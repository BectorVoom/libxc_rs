//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1068/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1068(t11239: f64, t3143: f64, t342: f64, t3298: f64, t989: f64, t4980: f64, t994: f64, t4995: f64, t1043: f64, t3153: f64, t3046: f64, t3286: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12077 = t11239 * t3143;
    let t12078 = t342 * t12077;
    let t12116 = t989 * t3298;
    let t12122 = t994 * t4980;
    let t12127 = t994 * t4995;
    let t12131 = t1043 * t3153;
    let t12146 = t3046 * t3286;
    (t12077, t12078, t12116, t12122, t12127, t12131, t12146)
}
