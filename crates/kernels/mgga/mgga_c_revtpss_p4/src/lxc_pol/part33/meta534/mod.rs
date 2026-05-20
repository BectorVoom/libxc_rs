//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta534<F: Float>(t28271: F, t572: F, t1459: F, t7953: F, t116: F, t7741: F, t670: F, t117: F, t28042: F, t27240: F, t27246: F, t27251: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t28273, t28275, t28276, t28277, t28279, t28280, t28282, t28330, t28333, t28335) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1886::<F>(t28271, t572, t1459, t7953, t116, t7741, t670, t117, t28042, t27240, t27246, t27251);
    (t28273, t28275, t28276, t28277, t28279, t28280, t28282, t28330, t28333, t28335)
}
