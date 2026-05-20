//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1927;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta376<F: Float>(t13312: F, t48: F, t10368: F, t1469: F, t2251: F, t2282: F, t4186: F, t606: F, t2258: F, t4210: F, t60: F, t10379: F, t13299: F, t13303: F, t13306: F, t1474: F, t1480: F, t2270: F, t2283: F, t2286: F, t4202: F, t4205: F, t44: F, t56: F, t614: F, t38: F, t1486: F, t2259: F, t4217: F, t607: F, t1471: F, t1487: F, t1494: F, t2252: F, t2260: F, t2263: F, t2312: F, t4196: F, t4218: F, t4238: F, t608: F, t641: F, t85: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13321, t13325, t13328, t13331, t13334) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1927::<F>(t13312, t48, t10368, t1469, t2251, t2282, t4186, t606, t2258, t4210, t60, t10379, t13299, t13303, t13306, t1474, t1480, t2270, t2283, t2286, t4202, t4205, t44, t56, t614);
        let (t13335, t13340, t13343, t13346, t13363) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1928::<F>(t13334, t38, t1486, t2251, t2259, t4217, t607, t1471, t1487, t1494, t2252, t2260, t2263, t2312, t4196, t4218, t4238, t608, t641, t85);
    (t13321, t13325, t13328, t13331, t13334, t13335, t13340, t13343, t13346, t13363)
}
