//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta375 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1235;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1236;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1237;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1238;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1239;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1240;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1241;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta375(t606: f64, t6092: f64, t904: f64, t128: f64, t4186: f64, t4578: f64, t6101: f64, t689: f64, t2852: f64, t5825: f64, t2850: f64, t2857: f64, t18281: f64, t905: f64, t11134: f64, t11304: f64, t15189: f64, t15209: f64, t15210: f64, t15211: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18926, t18928) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1235(t606, t6092, t904, t128);
        let (t18930, t18932) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1236(t4186, t4578, t904, t128);
        let t18934 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1237(t6101, t689);
        let (t18937, t18939) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1238(t2852, t5825, t606, t2850, t128);
        let (t18942, t18944) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1239(t2857, t5825, t606, t904, t128);
        let (t18946, t18948) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1240(t18281, t905, t904, t128);
        let t18950 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1241(t11134, t11304, t15189, t15209, t15210, t15211, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
    (t18926, t18928, t18930, t18932, t18934, t18937, t18939, t18942, t18944, t18946, t18948, t18950)
}
