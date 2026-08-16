//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1863;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta402(t12987: f64, t480: f64, t12629: f64, t482: f64, t371: f64, t372: f64, t127: f64, t3672: f64, t3671: f64, t140: f64, t3693: f64, t1222: f64, t10326: f64, t1225: f64, t1012: f64, t1235: f64, t1238: f64, t1261: f64, t12933: f64, t12938: f64, t12942: f64, t12945: f64, t12949: f64, t12953: f64, t12956: f64, t12960: f64, t12964: f64, t12967: f64, t12972: f64, t12976: f64, t12979: f64, t12985: f64, t3663: f64, t3667: f64, t3674: f64, t3711: f64, t3714: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12988, t12989, t12991, t12995, t12996, t12998, t12999) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1863(t12987, t480, t12629, t482, t371, t372, t127, t3672, t3671, t140, t3693, t1222);
        let (t13001, t13002, t13005) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1864(t10326, t1225, t1012, t1222, t1235, t1238, t1261, t12933, t12938, t12942, t12945, t12949, t12953, t12956, t12960, t12964, t12967, t12972, t12976, t12979, t12985, t12988, t12991, t12996, t12999, t3663, t3667, t3674, t3711, t3714);
    (t12988, t12989, t12991, t12995, t12996, t12998, t12999, t13001, t13002, t13005)
}
