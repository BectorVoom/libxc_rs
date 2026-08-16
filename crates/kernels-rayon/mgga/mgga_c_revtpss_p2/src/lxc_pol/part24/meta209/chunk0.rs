//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 949/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk949(t10111: f64, t22: f64, t870: f64, t10115: f64, t253: f64, t10867: f64, t251: f64, t2778: f64, t9303: f64, t871: f64, t9292: f64, t9646: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10939 = 0.19637199382202157274e-3_f64 * t10111 * t870 * t22;
    let t10948 = 0.11044544084478153697e-3_f64 * t10115 * t253;
    let t10952 = t10867 * t251;
    let t10969 = 0.26019841438354088051e-2_f64 * t9303 * t2778;
    let t10971 = 0.17073386770573548589e-1_f64 * t9292 * t871;
    let t10981 = t9646 * t251;
    (t10939, t10948, t10952, t10969, t10971, t10981)
}
