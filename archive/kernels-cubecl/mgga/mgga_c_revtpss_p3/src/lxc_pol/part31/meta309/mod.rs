//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1306;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1307;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta309<F: Float>(t2665: F, t9775: F, t2681: F, t820: F, t849: F, t857: F, t240: F, t2719: F, t2735: F, t2783: F, t2664: F, t808: F, t2693: F, t2710: F, t2713: F, t810: F, t9784: F, t9789: F, t235: F, t2453: F, t9794: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10719, t10722, t10723, t10726, t10744, t10745) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1306::<F>(t2665, t9775, t2681, t820, t849, t857, t240, t2719, t2735, t2783, t2664, t808);
        let (t10746, t10749, t10756, t10758, t10760, t10761) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1307::<F>(t10744, t10745, t2693, t2710, t2713, t810, t9784, t9789, t235, t2783, t2453, t2664, t9794);
    (t10719, t10722, t10723, t10726, t10744, t10746, t10749, t10756, t10758, t10760, t10761)
}
