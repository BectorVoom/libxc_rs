//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1497;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1498;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta278(t2664: f64, t808: f64, t10744: f64, t2693: f64, t2710: f64, t2713: f64, t810: f64, t9784: f64, t9789: f64, t235: f64, t2783: f64, t2453: f64, t9794: f64, t2475: f64, t72: f64, t245: f64, t2482: f64, t814: f64, t823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10745, t10746, t10749, t10756, t10758, t10759, t10760) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1497(t2664, t808, t10744, t2693, t2710, t2713, t810, t9784, t9789, t235, t2783, t2453);
        let (t10762, t10769, t10770, t10777) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1498(t2664, t9794, t10760, t2475, t72, t245, t2482, t814, t823);
    (t10745, t10746, t10749, t10756, t10758, t10759, t10760, t10762, t10769, t10770, t10777)
}
