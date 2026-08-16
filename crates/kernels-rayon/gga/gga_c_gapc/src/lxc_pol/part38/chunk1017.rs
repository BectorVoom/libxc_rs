//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1017/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1017(t1303: f64, t137: f64, t442: f64, t5971: f64, t1338: f64, t5964: f64, t5965: f64, t6: f64, t5972: f64, t1037: f64, t1431: f64, t1672: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20594 = t1303 * t137;
    let t20596 = t5971 * t20594 * t442;
    let t20602 = t1338 * t137;
    let t20604 = t5971 * t20602 * t442;
    let t20768 = t5964 * t5965 * t6;
    let t20773 = t5972 * t6;
    let t20774 = t5971 * t20773;
    let t20897 = t1037 * t1338;
    let t21049 = t1672 * t1431;
    (t20594, t20596, t20602, t20604, t20768, t20773, t20774, t20897, t21049)
}
