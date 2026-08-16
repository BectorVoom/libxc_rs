//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1989;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1990;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta579<F: Float>(t25299: F, t92894: F, t10073: F, t1958: F, t25390: F, t886: F, t1955: F, t25308: F, t2769: F, t7049: F, t786: F, t867: F, t2439: F, t25334: F, t887: F, t7036: F, t820: F, t844: F, t2482: F, t814: F, t10744: F, t2664: F, t7028: F, t25240: F, t2693: F, t2710: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t92895, t92905, t92917, t92921) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1989::<F>(t25299, t92894, t10073, t1958, t25390, t886, t1955, t25308, t2769, t7049, t786, t867);
        let (t92935, t92951, t92955, t92963, t92966) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1990::<F>(t2439, t25334, t887, t7036, t820, t844, t2482, t814, t10744, t2664, t7028, t25240, t2693, t2710);
    (t92895, t92905, t92917, t92921, t92935, t92951, t92955, t92963, t92966)
}
