//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta51 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk326;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk327;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk328;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk329;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk330;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta51(t300: f64, t311: f64, t912: f64, t938: f64, t941: f64, t946: f64, t955: f64, t961: f64, t965: f64, t974: f64, t315: f64, t964: f64, t972: f64, t973: f64, t902: f64, t908: f64, t341: f64, t340: f64, t338: f64, t378: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t978, t980, t981) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk326(t300, t311, t912, t938, t941, t946, t955, t961, t965, t974, t315);
        let (t983, t985, t986, t988, t989) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk327(t964, t972, t973, t981, t902, t908, t341);
        let (t992, t993) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk328(t340);
        let t994 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk329(t338, t993);
        let t995 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk330(t378, t994);
    (t978, t980, t981, t983, t985, t986, t988, t989, t992, t993, t994, t995)
}
