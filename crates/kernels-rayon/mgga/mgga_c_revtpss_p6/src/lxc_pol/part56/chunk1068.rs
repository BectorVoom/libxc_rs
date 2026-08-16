//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1068/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1068(t8937: f64, t96886: f64, t1276: f64, t13038: f64, t3598: f64, t33516: f64, t127: f64, t33495: f64, t33496: f64, t371: f64, t44841: f64, t8936: f64) -> (f64, f64, f64, f64) {
    let t124557 = t8937 * t96886;
    let t124560 = t124557 * t1276 * t13038 * t3598;
    let t124564 = t124557 * t33516 * t3598;
    let t124569 = t33495 * t371 * t127 * t33496;
    let t124571 = t8936 * t44841;
    (t124560, t124564, t124569, t124571)
}
