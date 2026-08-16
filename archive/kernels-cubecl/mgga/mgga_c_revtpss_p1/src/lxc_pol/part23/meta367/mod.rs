//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta367 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1685;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1686;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1687;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1688;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1689;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta367<F: Float>(t372: F, t4872: F, t3090: F, t4954: F, t15125: F, t15191: F, t4742: F, t993: F, t225: F, t366: F, t3224: F, t4845: F, t127: F, t371: F, t4852: F, t1025: F, t1646: F, t3056: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15584, t15618) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1685::<F>(t372, t4872, t3090, t4954);
        let (t15638, t15639, t15654) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1686::<F>(t15125, t15191, t4742, t993);
        let t15655 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1687::<F>(t15654, t225);
        let (t15656, t15662, t15666, t15668, t15669) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1688::<F>(t15655, t366, t3224, t4845, t127, t371, t4852, t1025, t1646, t3056);
        let t15670 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1689::<F>(t15669, t225);
    (t15584, t15618, t15638, t15639, t15654, t15655, t15656, t15662, t15666, t15668, t15669, t15670)
}
