//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 983/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk983(t15865: f64, t18005: f64, t17987: f64, t3235: f64, t4477: f64, t5101: f64, t4387: f64, t5324: f64, t1442: f64, t15776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18006 = t15865 * t18005;
    let t18009 = t3235 * t17987;
    let t18012 = t4477 * t5101;
    let t18013 = t4387 * t18012;
    let t18016 = t15865 * t5324;
    let t18019 = t15776 * t1442;
    (t18006, t18009, t18012, t18013, t18016, t18019)
}
