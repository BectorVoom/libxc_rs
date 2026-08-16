//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta866 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3021;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3022;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta866(t14792: f64, t50768: f64, t50769: f64, t14688: f64, t40731: f64, t10777: f64, t14671: f64, t14686: f64, t2754: f64, t14749: f64, t221: f64, t10703: f64, t2674: f64, t4398: f64, t9323: f64, t4302: f64, t9586: f64, t10612: f64, t4311: f64, t14330: f64, t14369: f64, t2251: f64, t14440: f64, t2398: f64, t2258: f64, t4401: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50771, t50773, t50784, t50791) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3021(t14792, t50768, t50769, t14688, t40731, t10777, t14671, t14686, t2754, t14749, t221, t10703, t2674);
        let (t50852, t50856, t50865, t50868, t50873, t50878) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3022(t4398, t9323, t4302, t9586, t10612, t4311, t14330, t14369, t2251, t14440, t2398, t2258, t4401);
    (t50771, t50773, t50784, t50791, t50852, t50856, t50865, t50868, t50873, t50878)
}
