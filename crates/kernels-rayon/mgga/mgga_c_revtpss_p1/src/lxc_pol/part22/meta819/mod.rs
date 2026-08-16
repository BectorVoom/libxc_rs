//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta819 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2931;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2932;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta819(t136: f64, t2457: f64, t5774: f64, t9674: f64, t10175: f64, t14079: f64, t10073: f64, t13731: f64, t3915: f64, t5721: f64, t9288: f64, t2439: f64, t3895: f64, t5775: f64, t14066: f64, t213: f64, t14109: f64, t47603: f64, t9681: f64, t14268: f64, t686: f64, t72: f64, t14293: f64, t9664: f64, t1444: f64, t2782: f64, t4075: f64, t556: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47885, t47893, t47899, t47904, t47907) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2931(t136, t2457, t5774, t9674, t10175, t14079, t10073, t13731, t3915, t5721, t9288, t2439, t3895, t5775);
        let (t47909, t47913, t47918, t47920, t47926) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2932(t14066, t213, t14109, t47603, t9681, t14268, t3915, t686, t72, t14293, t9664, t1444, t2782, t4075, t556, t5774);
    (t47885, t47893, t47899, t47904, t47907, t47909, t47913, t47918, t47920, t47926)
}
