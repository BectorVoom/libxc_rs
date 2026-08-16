//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2380/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2380(t10733: f64, t9775: f64, t10716: f64, t10741: f64, t10665: f64, t243: f64, t231: f64, t2661: f64, t2662: f64, t10737: f64, t2652: f64, t212: f64, t2237: f64, t225: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40475 = t9775 * t10733;
    let t40477 = t10716 * t10741;
    let t40479 = t243 * t10665;
    let t40482 = t2661 * t2662 * t40479 * t231;
    let t40484 = t2652 * t10737;
    let t40488 = t816 * t2237 * t212 * t225;
    (t40475, t40477, t40479, t40482, t40484, t40488)
}
