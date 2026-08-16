//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1263/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1263(t2719: f64, t820: f64, t844: f64, t2482: f64, t814: f64, t11509: f64, t2988: f64, t4900: f64, t999: f64, t4894: f64, t245: f64, t4890: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14923 = t820 * t2719 * t844;
    let t14931 = t2482 * t2719 * t814;
    let t15542 = t11509 * t2988;
    let t15604 = t4900 * t999;
    let t15609 = t4894 * t999;
    let t15687 = t4890 * t245;
    (t14923, t14931, t15542, t15604, t15609, t15687)
}
