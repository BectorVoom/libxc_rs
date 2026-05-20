//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2049;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2050;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta479<F: Float>(t2258: F, t4578: F, t904: F, t128: F, t11150: F, t1469: F, t2251: F, t2850: F, t4573: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15149, t15150, t15151) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2049::<F>(t2258, t4578, t904, t128);
        let (t15153, t15154, t15155, t15156) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2050::<F>(t11150, t1469, t2251, t2850, t128);
        let (t15158, t15159, t15160) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2051::<F>(t2251, t4573, t904, t128);
    (t15149, t15150, t15151, t15153, t15154, t15155, t15156, t15158, t15159, t15160)
}
