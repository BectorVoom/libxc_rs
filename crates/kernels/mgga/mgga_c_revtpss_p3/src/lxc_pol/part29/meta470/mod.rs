//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1732;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1733;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta470<F: Float>(t26506: F, t7064: F, t2061: F, t2722: F, t25416: F, t2723: F, t231: F, t7076: F, t136: F, t2066: F, t2457: F, t25299: F, t25365: F, t7407: F, t1956: F, t213: F, t25383: F, t257: F, t26437: F, t26439: F, t26441: F, t26448: F, t26475: F, t26483: F, t26486: F, t26489: F, t26493: F, t26498: F, t26500: F, t26502: F, t7067: F, t7070: F, t7415: F, t7424: F) -> (F, F, F, F, F, F, F, F) {
        let (t26508, t26511, t26515, t26518, t26519) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1732::<F>(t26506, t7064, t2061, t2722, t25416, t2723, t231, t7076, t136, t2066, t2457);
        let (t26521, t26522, t26524) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1733::<F>(t25299, t26519, t25365, t7407, t1956, t213, t25383, t257, t26437, t26439, t26441, t26448, t26475, t26483, t26486, t26489, t26493, t26498, t26500, t26502, t26508, t26511, t26515, t7067, t7070, t7415, t7424);
    (t26508, t26511, t26515, t26518, t26519, t26521, t26522, t26524)
}
