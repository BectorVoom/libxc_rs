//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2453;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2454;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta660(t11858: f64, t16048: f64, t11859: f64, t11861: f64, t11922: f64, t11927: f64, t11929: f64, t1065: f64, t215: f64, t1063: f64, t247: f64, t906: f64, t11986: f64, t2858: f64, t11744: f64, t3106: f64, t373: f64, t675: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42765, t42769, t42772, t42778, t42781) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2453(t11858, t16048, t11859, t11861, t11922, t11927, t11929, t1065, t215, t1063, t247, t906);
        let (t42785, t42788, t42792, t42793) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2454(t1063, t11986, t247, t2858, t11744, t3106, t373, t675, t828);
    (t42765, t42769, t42772, t42778, t42781, t42785, t42788, t42792, t42793)
}
