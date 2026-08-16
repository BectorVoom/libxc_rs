//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1508;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1509;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1510;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta279<F: Float>(t10115: F, t557: F, t10024: F, t268: F, t543: F, t4101: F, t1429: F, t9292: F, t3964: F, t4096: F, t9285: F, t1385: F, t4066: F, t1398: F, t215: F, t2453: F, t4100: F, t281: F, t68: F, t10080: F, t10082: F, t10085: F, t10090: F, t10098: F, t10102: F, t10105: F, t10109: F, t10114: F, t1399: F, t4057: F, t4114: F, t4118: F, t5755: F, t820: F, t9912: F, t9995: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10117, t10119, t10120, t10126, t10129, t10130) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1508::<F>(t10115, t557, t10024, t268, t543, t4101, t1429, t9292, t3964, t4096, t9285, t1385, t4066);
        let (t10136, t10137, t10139) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1509::<F>(t1398, t215, t268, t543, t4101, t2453, t4100);
        let (t10142, t10143, t10145) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1510::<F>(t1398, t281, t543, t68, t10139, t10080, t10082, t10085, t10090, t10098, t10102, t10105, t10109, t10114, t10117, t10120, t10126, t10129, t10130, t10137, t1399, t4057, t4114, t4118, t5755, t820, t9912, t9995);
    (t10117, t10119, t10120, t10126, t10129, t10130, t10136, t10137, t10139, t10142, t10143, t10145)
}
