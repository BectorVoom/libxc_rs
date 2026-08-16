//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1081/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1081(t1260: f64, t3666: f64, t12640: f64, t225: f64, t480: f64, t1236: f64, t371: f64, t676: f64, t1235: f64, t12627: f64, t1226: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12956 = t3666 * t1260;
    let t12966 = t12640 * t225;
    let t12967 = t12966 * t480;
    let t12984 = t371 * t676 * t1236;
    let t12985 = t1235 * t12984;
    let t12987 = t12627 * t225;
    let t13011 = t697 * t1226;
    (t12956, t12966, t12967, t12985, t12987, t13011)
}
