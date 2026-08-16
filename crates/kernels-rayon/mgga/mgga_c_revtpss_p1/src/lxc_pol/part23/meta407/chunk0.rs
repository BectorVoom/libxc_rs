//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1781/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1781(t18263: f64, t707: f64, t10605: f64, t6002: f64, t2411: f64, t6079: f64, t10446: f64, t5819: f64, t2375: f64, t5825: f64, t13309: f64, t13310: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18265 = 4.0_f64 * t18263 * t707;
    let t18267 = 12.0_f64 * t10605 * t6002;
    let t18268 = t6079 * t2411;
    let t18272 = t10446 * t5819;
    let t18277 = t2375 * t5825;
    let t18280 = -t13309 - t13310;
    (t18265, t18267, t18268, t18272, t18277, t18280)
}
