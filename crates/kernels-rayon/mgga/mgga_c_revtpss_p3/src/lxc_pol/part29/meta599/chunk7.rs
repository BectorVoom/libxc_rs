//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2046/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2046(t2118: f64, t5789: f64, t1464: f64, t8113: f64, t1913: f64, t7560: f64, t2110: f64, t5808: f64, t1455: f64, t8130: f64, t1921: f64, t7541: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104071 = 2.0_f64 * t5789 * t2118;
    let t104073 = 2.0_f64 * t8113 * t1464;
    let t104077 = 2.0_f64 * t1913 * t7560;
    let t104079 = 2.0_f64 * t2110 * t5808;
    let t104081 = 2.0_f64 * t1455 * t8130;
    let t104083 = 2.0_f64 * t7541 * t1921;
    (t104071, t104073, t104077, t104079, t104081, t104083)
}
