//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1184/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1184(t14662: f64, t231: f64, t10943: f64, t4364: f64, t4365: f64, t124: f64, t1558: f64, t10779: f64, t2749: f64, t10777: f64, t125: f64, t4423: f64) -> (f64, f64, f64, f64, f64) {
    let t14663 = t14662 * t231;
    let t14668 = t4364 * t4365 * t10943;
    let t14671 = t124 * t1558;
    let t14673 = t10779 * t14671 * t2749;
    let t14675 = 0.10164000561857065645e-3_f64 * t10777 * t14673;
    let t14676 = t125 * t4423;
    (t14663, t14668, t14671, t14675, t14676)
}
