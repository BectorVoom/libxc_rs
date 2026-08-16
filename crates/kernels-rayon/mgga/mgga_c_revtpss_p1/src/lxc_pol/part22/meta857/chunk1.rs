//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3004/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3004(t10777: f64, t10779: f64, t14671: f64, t14872: f64, t10811: f64, t14682: f64, t14804: f64, t14923: f64, t4457: f64, t837: f64, t14853: f64, t2652: f64) -> (f64, f64, f64, f64, f64) {
    let t50325 = t10777 * t10779 * t14671 * t14872;
    let t50328 = t10811 * t14682;
    let t50347 = t14923 * t14804;
    let t50351 = t10777 * t10779 * t4457 * t837;
    let t50353 = t2652 * t14853;
    (t50325, t50328, t50347, t50351, t50353)
}
