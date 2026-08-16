//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 859/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk859(t1955: f64, t25920: f64, t545: f64, t9656: f64, t4075: f64, t7282: f64, t1385: f64, t2022: f64, t1426: f64, t10073: f64, t2453: f64, t7283: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25921 = t1955 * t25920;
    let t25924 = t9656 * t545;
    let t25929 = t7282 * t4075;
    let t25930 = t1955 * t25929;
    let t25931 = t1385 * t2022;
    let t25937 = t1426 * t545;
    let t25938 = t25937 * t2022;
    let t25939 = t7282 * t25938;
    let t25941 = 0.24093411633903331839e-3_f64 * t10073 * t25939;
    let t25944 = t2453 * t7283;
    (t25921, t25924, t25930, t25931, t25937, t25938, t25941, t25944)
}
