//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1296/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1296(t11210: f64, t11657: f64, t15650: f64, t10287: f64, t190: f64, t7108: f64, t959: f64, t10153: f64, t3727: f64, t6182: f64, t3243: f64, t6188: f64) -> (f64, f64, f64, f64) {
    let t35979 = t11657 * t11210 * t15650;
    let t35983 = t10287 * t190 * t959 * t7108;
    let t35986 = t10153 * t3727 * t6182;
    let t35989 = t3243 * t3727 * t6188;
    (t35979, t35983, t35986, t35989)
}
