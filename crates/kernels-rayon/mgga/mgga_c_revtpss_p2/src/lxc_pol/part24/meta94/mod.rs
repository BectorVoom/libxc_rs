//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta94 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk540;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk541;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk542;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk543;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk544;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta94(t2966: f64, t302: f64, t310: f64, t2846: f64, t320: f64, t963: f64, t315: f64, t2904: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2967, t2968) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk540(t2966, t302);
        let (t2969, t2970) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk541(t310);
        let (t2974, t2985, t2986) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk542(t2846, t320, t963);
        let (t2987, t2994, t3001, t3010) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk543(t2986, t315, t2846, t2904, t963);
        let t3011 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk544(t3010);
    (t2967, t2968, t2969, t2970, t2974, t2985, t2986, t2987, t2994, t3001, t3010, t3011)
}
