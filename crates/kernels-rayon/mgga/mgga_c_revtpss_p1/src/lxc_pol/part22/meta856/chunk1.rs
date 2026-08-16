//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3002/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3002(t10995: f64, t122: f64, t14982: f64, t2466: f64, t10777: f64, t10779: f64, t1548: f64, t2646: f64, t10868: f64, t820: f64, t844: f64, t14896: f64) -> (f64, f64, f64, f64) {
    let t50259 = t10995 * t14982 * t122 * t2466;
    let t50292 = t10777 * t10779 * t1548 * t2646;
    let t50295 = t820 * t10868 * t844;
    let t50296 = t50295 * t14896;
    (t50259, t50292, t50295, t50296)
}
