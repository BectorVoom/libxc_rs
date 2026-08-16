//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1188/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1188(t2242: f64, t2251: f64, t2311: f64, t644: f64, t77: f64, t2315: f64, t640: f64, t10410: f64, t84: f64, t2258: f64, t10327: f64, t603: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92639 = t2242 * t2251;
    let t92654 = t77 * t2311 * t644;
    let t92658 = t77 * t640 * t2315;
    let t92662 = t77 * t84 * t10410;
    let t92672 = t77 * t84 * t2258;
    let t92674 = t603 * t10327;
    (t92639, t92654, t92658, t92662, t92672, t92674)
}
