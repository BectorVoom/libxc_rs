//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1684/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1684(t12587: f64, t3794: f64, t3798: f64, t45282: f64, t45296: f64, t45298: f64, t45302: f64, t45306: f64, t45310: f64, t45312: f64, t45316: f64, t45318: f64, t45321: f64, t45323: f64, t45326: f64, t5023: f64) -> f64 {
    let t45908 = 12.0_f64 * t12587 * t3794 * t3798 * t5023 - t45282 + t45296 - t45298 - t45302 + t45306 - t45310 + t45312 - t45316 - t45318 - t45321 - t45323 + t45326;
    t45908
}
