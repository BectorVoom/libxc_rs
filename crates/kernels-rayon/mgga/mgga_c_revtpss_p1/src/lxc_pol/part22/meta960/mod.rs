//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta960 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3221;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3222;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta960(t18299: f64, t750: f64, t49911: f64, t4537: f64, t18298: f64, t705: f64, t707: f64, t14749: f64, t14767: f64, t198: f64, t207: f64, t2411: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t4541: f64, t4546: f64, t18281: f64, t706: f64, t39737: f64, t190: f64, t60754: f64, t18838: f64, t892: f64, t11075: f64, t14375: f64, t18435: f64, t2403: f64, t2404: f64, t39540: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t39756: f64, t5962: f64, t775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61115, t61116, t61124, t61125) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3221(t18299, t750, t49911, t4537, t18298, t705, t707, t14749, t14767, t198, t207, t2411, t39483, t39520, t39528, t39531, t39534, t39537, t4541, t4546);
        let (t61131, t61135, t61138, t61146) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3222(t18281, t706, t750, t39737, t190, t60754, t18838, t892, t11075, t14375, t18435, t198, t2403, t2404, t39540, t39741, t39744, t39747, t39750, t39756, t4541, t5962, t775);
    (t61115, t61116, t61124, t61125, t61131, t61135, t61138, t61146)
}
