//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta904 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2899;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2900;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2901;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2902;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2903;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2904;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2905;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta904(t15144: f64, t5825: f64, t128: f64, t904: f64, t18281: f64, t4578: f64, t41361: f64, t41520: f64, t51978: f64, t52337: f64, t63276: f64, t63278: f64, t77499: f64, t77503: f64, t77505: f64, t77507: f64, t77509: f64, t77515: f64, t77518: f64, t77521: f64, t77527: f64, t77531: f64, t77535: f64, t77539: f64, t23500: f64, t689: f64, t23504: f64, t22688: f64, t41270: f64, t606: f64, t11142: f64, t18903: f64, t4186: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77541, t77543) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2899(t15144, t5825, t128, t904);
        let (t77545, t77547) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2900(t18281, t4578, t128, t904);
        let t77549 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2901(t41361, t41520, t51978, t52337, t63276, t63278, t77499, t77503, t77505, t77507, t77509, t77515, t77518, t77521, t77527, t77531, t77535, t77539, t77543, t77547);
        let t77559 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2902(t23500, t689);
        let t77561 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2903(t23504, t689);
        let (t77564, t77566) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2904(t22688, t41270, t606, t11142, t128);
        let (t77568, t77570) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2905(t18903, t4186, t11142, t128);
    (t77541, t77543, t77545, t77547, t77549, t77559, t77561, t77564, t77566, t77568, t77570)
}
