//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta904 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2899;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2900;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2901;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2902;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2903;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2904;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2905;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta904<F: Float>(t15144: F, t5825: F, t128: F, t904: F, t18281: F, t4578: F, t41361: F, t41520: F, t51978: F, t52337: F, t63276: F, t63278: F, t77499: F, t77503: F, t77505: F, t77507: F, t77509: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F, t77539: F, t23500: F, t689: F, t23504: F, t22688: F, t41270: F, t606: F, t11142: F, t18903: F, t4186: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t77541, t77543) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2899::<F>(t15144, t5825, t128, t904);
        let (t77545, t77547) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2900::<F>(t18281, t4578, t128, t904);
        let t77549 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2901::<F>(t41361, t41520, t51978, t52337, t63276, t63278, t77499, t77503, t77505, t77507, t77509, t77515, t77518, t77521, t77527, t77531, t77535, t77539, t77543, t77547);
        let t77559 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2902::<F>(t23500, t689);
        let t77561 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2903::<F>(t23504, t689);
        let (t77564, t77566) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2904::<F>(t22688, t41270, t606, t11142, t128);
        let (t77568, t77570) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2905::<F>(t18903, t4186, t11142, t128);
    (t77541, t77543, t77545, t77547, t77549, t77559, t77561, t77564, t77566, t77568, t77570)
}
