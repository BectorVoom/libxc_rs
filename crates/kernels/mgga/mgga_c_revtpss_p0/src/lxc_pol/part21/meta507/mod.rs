//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2126;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2127;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2128;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2129;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta507<F: Float>(t15984: F, t3091: F, t1014: F, t140: F, t4579: F, t1011: F, t11672: F, t11675: F, t11881: F, t11886: F, t12004: F, t15952: F, t15959: F, t15965: F, t15970: F, t15975: F, t1675: F, t3127: F, t4783: F, t4892: F, t4899: F, t3252: F, t4574: F, t15145: F, t4915: F, t15149: F, t15154: F, t4919: F, t15130: F, t15135: F, t1012: F, t11821: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15986, t15987) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2126::<F>(t15984, t3091, t1014, t140);
        let (t15988, t15991) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2127::<F>(t15987, t4579, t1011, t11672, t11675, t11881, t11886, t12004, t15952, t15959, t15965, t15970, t15975, t15986, t1675, t3091, t3127, t4783, t4892, t4899);
        let t15993 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2128::<F>(t140, t3252);
        let (t15994, t15996, t15997, t16000, t16003, t16006, t16009, t16012) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2129::<F>(t15993, t4574, t1011, t15145, t4915, t15149, t15154, t4919, t15130, t15135, t1012, t11821);
    (t15987, t15988, t15991, t15993, t15994, t15996, t15997, t16000, t16003, t16006, t16009, t16012)
}
