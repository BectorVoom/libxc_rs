//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1189/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1189(t19778: f64, t19805: f64, t16638: f64, t10535: f64, t496: f64, t501: f64, t16626: f64, t16631: f64, t16701: f64, t16873: f64, t19757: f64, t19759: f64, t19776: f64, t19798: f64, t19804: f64, t19823: f64, t28970: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29118 = 3.0_f64 * t19778;
    let t29119 = 360.0_f64 * t19805;
    let t29120 = 60.0_f64 * t16638;
    let t29121 = t496 * t10535;
    let t29122 = 4.0_f64 * t29121;
    let t29123 = t501 * t10535;
    let t29124 = 4.0_f64 * t29123;
    let t29125 = t28970 + t19757 + t19759 + t19776 + t29118 + t16626 - t16631 + t19798 - t19804 - t29119 + t29120 + t16873 + t29122 - t29124 + t16701 - t19823;
    (t29118, t29119, t29120, t29122, t29124, t29125)
}
