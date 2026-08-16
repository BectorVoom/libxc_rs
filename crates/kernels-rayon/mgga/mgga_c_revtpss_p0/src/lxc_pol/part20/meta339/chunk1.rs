//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1265/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1265(t3105: f64, t3204: f64, t12116: f64, t4891: f64, t3133: f64, t3154: f64, t11243: f64, t72: f64, t3088: f64) -> (f64, f64, f64, f64, f64) {
    let t15728 = t3204 * t3105;
    let t15758 = t12116 * t4891;
    let t15785 = t3154 * t3133;
    let t15904 = t11243 * t72;
    let t15905 = t3088 * t15904;
    (t15728, t15758, t15785, t15904, t15905)
}
