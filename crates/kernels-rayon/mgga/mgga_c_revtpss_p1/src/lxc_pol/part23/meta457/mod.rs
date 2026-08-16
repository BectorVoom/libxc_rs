//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1892;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1893;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta457(t1678: f64, t3316: f64, t342: f64, t6299: f64, t73: f64, t4976: f64, t1082: f64, t19414: f64, t1045: f64, t999: f64, t6271: f64, t3117: f64, t19501: f64, t3095: f64, t3092: f64, t1043: f64, t3155: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19607, t19608, t19611) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1892(t1678, t3316, t342, t6299, t73);
        let (t19612, t19617, t19620) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1893(t19611, t4976, t1082, t19414, t1045, t999);
        let (t19621, t19622, t19625, t19626, t19634) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1894(t19620, t6271, t3117, t19501, t3095, t3092, t1043, t3155);
    (t19607, t19608, t19611, t19612, t19617, t19620, t19621, t19622, t19625, t19626, t19634)
}
