//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta48 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk328;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk329;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk330;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk331;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk332;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta48(t315: f64, t964: f64, t902: f64, t928: f64, t908: f64, t919: f64, t924: f64, t932: f64, t323: f64, t300: f64, t311: f64, t912: f64, t938: f64, t941: f64, t946: f64, t955: f64, t961: f64, t341: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t965, t972) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk328(t315, t964, t902, t928, t908, t919, t924, t932);
        let t973 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk329(t323);
        let t974 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk330(t972, t973);
        let (t978, t980, t981) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk331(t300, t311, t912, t938, t941, t946, t955, t961, t965, t974, t315);
        let (t983, t985, t988, t989) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk332(t964, t972, t973, t981, t902, t908, t341);
    (t965, t972, t973, t974, t978, t980, t981, t983, t985, t988, t989)
}
