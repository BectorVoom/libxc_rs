//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1770;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1771;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1772;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta324<F: Float>(t2664: F, t808: F, t10744: F, t2693: F, t2710: F, t2713: F, t810: F, t9784: F, t9789: F, t235: F, t2783: F, t2453: F, t9794: F, t2475: F, t72: F, t245: F, t2482: F, t814: F, t823: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10745, t10746, t10749, t10756, t10758, t10759, t10760) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1770::<F>(t2664, t808, t10744, t2693, t2710, t2713, t810, t9784, t9789, t235, t2783, t2453);
        let (t10762, t10769, t10770) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1771::<F>(t2664, t9794, t10760, t2475, t72, t245);
        let t10777 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1772::<F>(t2482, t814, t823);
    (t10745, t10746, t10749, t10756, t10758, t10759, t10760, t10762, t10769, t10770, t10777)
}
