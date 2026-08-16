//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1831/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1831(t1043: f64, t7161: f64, t1089: f64, t378: f64, t7150: f64, t8521: f64) -> (f64, f64, f64) {
    let t25606 = t7161 * t1043;
    let t25607 = t25606 * t1089;
    let t25610 = t7150 * t378;
    let t25611 = t25610 * t8521;
    (t25607, t25610, t25611)
}
