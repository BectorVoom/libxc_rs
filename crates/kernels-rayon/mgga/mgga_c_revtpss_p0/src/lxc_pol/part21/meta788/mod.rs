//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta788 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2836;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta788(t15177: f64, t698: f64, t15180: f64, t15129: f64, t2258: f64, t141: f64, t2908: f64, t11144: f64, t2251: f64, t4186: f64, t11341: f64, t51851: f64, t15162: f64, t51873: f64, t15165: f64, t51847: f64, t930: f64, t41246: f64, t41267: f64, t41275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51921, t51923, t51925, t51927, t51930, t51932, t51935) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2836(t15177, t698, t15180, t15129, t2258, t141, t2908, t11144, t2251, t4186, t11341, t51851);
        let (t51937, t51940, t51942, t51945, t51949) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2837(t15162, t698, t141, t2908, t51873, t15165, t51847, t930, t41246, t41267, t41275, t51921, t51923, t51927, t51932, t51935);
    (t51921, t51923, t51925, t51927, t51930, t51932, t51935, t51937, t51940, t51942, t51945, t51949)
}
