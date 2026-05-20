//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta790 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2847;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2848;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta790<F: Float>(t141: F, t41294: F, t51856: F, t51865: F, t930: F, t51869: F, t51861: F, t11150: F, t2251: F, t4186: F, t2908: F, t10356: F, t1469: F, t41270: F, t11341: F, t15129: F, t41361: F, t41363: F, t41369: F, t51978: F, t138: F, t140: F, t240: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t51981, t51984, t51987, t51990, t51993, t51995, t51998) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2847::<F>(t141, t41294, t51856, t51865, t930, t51869, t51861, t11150, t2251, t4186, t2908, t10356, t1469, t41270);
        let (t52000, t52002, t52004, t52009) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2848::<F>(t11341, t141, t51998, t15129, t2251, t930, t41361, t41363, t41369, t51978, t51981, t51984, t51987, t51990, t51995);
        let t52011 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2849::<F>(t138, t140, t240);
    (t51981, t51984, t51987, t51990, t51993, t51995, t51998, t52000, t52002, t52004, t52009, t52011)
}
