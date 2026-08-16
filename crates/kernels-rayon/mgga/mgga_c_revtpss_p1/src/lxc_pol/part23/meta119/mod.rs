//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta119 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk774;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk775;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk776;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk777;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta119(t1036: f64, t3148: f64, t3141: f64, t3153: f64, t357: f64, t1038: f64, t1052: f64, t1033: f64, t127: f64, t246: f64, t1046: f64, t1041: f64, t283: f64, t905: f64, t66: f64, t1020: f64, t1062: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3160, t3161, t3162) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk774(t1036, t3148, t3141, t3153, t357);
        let (t3168, t3169) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk775(t1038, t1052, t1036, t1033);
        let t3172 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk776(t127, t246);
        let (t3173, t3174, t3181) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk777(t1046, t3172, t1041, t283, t905);
        let (t3182, t3188) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk778(t3181, t66, t1020, t1062);
    (t3160, t3161, t3162, t3168, t3169, t3172, t3173, t3174, t3181, t3182, t3188)
}
