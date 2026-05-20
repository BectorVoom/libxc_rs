//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1907;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1908;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1909;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta530<F: Float>(t4181: F, t603: F, t4187: F, t38: F, t7714: F, t2247: F, t1493: F, t644: F, t77: F, t13272: F, t6957: F, t4173: F, t607: F, t7705: F, t1497: F, t1927: F, t1926: F, t1470: F, t1928: F, t25099: F, t25157: F, t25162: F, t25164: F, t6958: F, t6960: F, t6963: F, t6974: F, t6978: F, t7706: F, t7709: F, t7716: F, t7720: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t28116, t28119, t28126, t28127, t28133, t28138, t28141) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1907::<F>(t4181, t603, t4187, t38, t7714, t2247, t1493, t644, t77, t13272, t6957, t4173, t607);
        let (t28147, t28150, t28151, t28154) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1908::<F>(t644, t77, t7705, t1497, t1927, t1926, t1470, t2247);
        let t28157 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1909::<F>(t1928, t25099, t25157, t25162, t25164, t28116, t28119, t28127, t28133, t28138, t28141, t28147, t28151, t28154, t6958, t6960, t6963, t6974, t6978, t7706, t7709, t7716, t7720);
    (t28116, t28119, t28126, t28127, t28133, t28138, t28141, t28147, t28150, t28151, t28154, t28157)
}
