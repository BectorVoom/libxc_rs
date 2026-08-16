//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1786;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1787;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta376(t12292: f64, t12296: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64, t1132: f64, t409: f64, t416: f64, t1134: f64, t3391: f64, t406: f64, t12252: f64, t12259: f64, t12261: f64, t12263: f64, t12265: f64, t12271: f64, t12275: f64, t12279: f64, t12284: f64, t12289: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12322, t12323) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1786(t12292, t12296, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320, t1132);
        let (t12327, t12328, t12329, t12331, t12332, t12334) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1787(t409, t416, t1134, t3391, t406, t12252, t12259, t12261, t12263, t12265, t12271, t12275, t12279, t12284, t12289, t12292, t12323);
    (t12322, t12323, t12327, t12328, t12329, t12331, t12332, t12334)
}
