//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2046;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2047;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2048;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta478<F: Float>(t2258: F, t4573: F, t2850: F, t128: F, t11144: F, t1469: F, t2251: F, t11142: F, t2857: F, t4186: F, t606: F, t904: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15135, t15136, t15137) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2046::<F>(t2258, t4573, t2850, t128);
        let (t15139, t15140, t15141, t15142) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2047::<F>(t11144, t1469, t2251, t11142, t128);
        let (t15144, t15145, t15146, t15147) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2048::<F>(t2857, t4186, t606, t904, t128);
    (t15135, t15136, t15137, t15139, t15140, t15141, t15142, t15144, t15145, t15146, t15147)
}
