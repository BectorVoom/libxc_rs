//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1293/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1293(t1450: f64, t2014: f64, t2033: f64, t9400: f64, t10259: f64, t572: f64, t7330: f64, t117: f64, t94991: f64, t116: f64, t25832: f64, t670: f64) -> (f64, f64, f64, f64) {
    let t95108 = 6.0_f64 * t2014 * t9400 * t2033 * t1450;
    let t95131 = 6.0_f64 * t572 * t7330 * t10259;
    let t95136 = 3.0_f64 * t572 * t117 * t94991;
    let t95137 = t116 * t25832;
    let t95140 = 18.0_f64 * t572 * t95137 * t670;
    (t95108, t95131, t95136, t95140)
}
