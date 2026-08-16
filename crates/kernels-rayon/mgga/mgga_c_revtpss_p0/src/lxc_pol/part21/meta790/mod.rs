//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta790 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2847;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2848;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta790(t141: f64, t41294: f64, t51856: f64, t51865: f64, t930: f64, t51869: f64, t51861: f64, t11150: f64, t2251: f64, t4186: f64, t2908: f64, t10356: f64, t1469: f64, t41270: f64, t11341: f64, t15129: f64, t41361: f64, t41363: f64, t41369: f64, t51978: f64, t138: f64, t140: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51981, t51984, t51987, t51990, t51993, t51995, t51998) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2847(t141, t41294, t51856, t51865, t930, t51869, t51861, t11150, t2251, t4186, t2908, t10356, t1469, t41270);
        let (t52000, t52002, t52004, t52009) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2848(t11341, t141, t51998, t15129, t2251, t930, t41361, t41363, t41369, t51978, t51981, t51984, t51987, t51990, t51995);
        let t52011 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2849(t138, t140, t240);
    (t51981, t51984, t51987, t51990, t51993, t51995, t51998, t52000, t52002, t52004, t52009, t52011)
}
