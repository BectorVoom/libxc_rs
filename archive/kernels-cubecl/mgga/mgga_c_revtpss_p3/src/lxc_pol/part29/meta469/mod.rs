//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1730;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta469<F: Float>(t25387: F, t26485: F, t2061: F, t2771: F, t25317: F, t7398: F, t886: F, t7071: F, t2062: F, t867: F, t786: F, t2467: F, t25431: F, t26482: F, t225: F, t26473: F, t2470: F, t7406: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t26486, t26488, t26489, t26492, t26493, t26496, t26497) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1730::<F>(t25387, t26485, t2061, t2771, t25317, t7398, t886, t7071, t2062, t867, t786);
        let (t26498, t26500, t26502, t26506) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1731::<F>(t2467, t26497, t25431, t26482, t225, t26473, t2470, t7406);
    (t26486, t26488, t26489, t26492, t26493, t26496, t26497, t26498, t26500, t26502, t26506)
}
