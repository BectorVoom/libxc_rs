//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1890;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1891;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta415<F: Float>(t13334: F, t38: F, t1486: F, t2251: F, t2259: F, t4217: F, t607: F, t1471: F, t1487: F, t1494: F, t2252: F, t2260: F, t2263: F, t2312: F, t4196: F, t4218: F, t4238: F, t608: F, t641: F, t85: F, t10389: F, t1469: F, t2299: F, t4186: F, t10398: F, t2306: F, t13312: F, t2258: F, t4227: F, t4232: F, t606: F, t633: F, t637: F, t77: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13335, t13340, t13343, t13346, t13363) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1890::<F>(t13334, t38, t1486, t2251, t2259, t4217, t607, t1471, t1487, t1494, t2252, t2260, t2263, t2312, t4196, t4218, t4238, t608, t641, t85);
        let (t13368, t13371, t13378, t13381, t13388) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1891::<F>(t10389, t1469, t2299, t4186, t10398, t2306, t13312, t2251, t2258, t4227, t4232, t606, t633, t637);
        let (t13389, t13392) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1892::<F>(t13388, t77, t1469, t2258);
    (t13335, t13340, t13343, t13346, t13363, t13368, t13371, t13378, t13381, t13389, t13392)
}
