//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta155 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk950;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk951;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk952;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta155<F: Float>(t1469: F, t189: F, t606: F, t4401: F, t2623: F, t2621: F, t2628: F, t2632: F, t4307: F, t4310: F, t4313: F, t4316: F, t4394: F, t4396: F, t4397: F, t4400: F, t225: F, t4376: F, t227: F, t73: F, t1544: F, t853: F, t775: F, t4343: F, t832: F, t1553: F, t1555: F, t229: F, t830: F, t833: F, t231: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4402, t4403, t4405, t4406, t4407) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk950::<F>(t1469, t189, t606, t4401, t2623, t2621, t2628, t2632, t4307, t4310, t4313, t4316, t4394, t4396, t4397, t4400);
        let (t4409, t4415, t4416) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk951::<F>(t225, t4376, t4407, t227, t73, t1544, t853);
        let (t4417, t4420, t4423) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk952::<F>(t4416, t775, t4343, t832, t1553, t1555, t227, t229, t4409, t4415, t830, t833);
        let t4424 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk953::<F>(t231, t4423);
    (t4402, t4403, t4405, t4406, t4409, t4415, t4416, t4417, t4420, t4423, t4424)
}
