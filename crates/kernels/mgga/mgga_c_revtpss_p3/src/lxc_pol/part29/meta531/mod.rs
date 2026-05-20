//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1860;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta531<F: Float>(t2247: F, t2251: F, t68: F, t26205: F, t6963: F, t45972: F, t7342: F, t10309: F, t26178: F, t25159: F, t2047: F, t92569: F, t116: F, t26209: F, t94973: F, t26375: F, t531: F, t198: F, t206: F, t7427: F, t2411: F, t26580: F, t25373: F, t26550: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t95310, t95314, t95316, t95319, t95320, t95340) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1860::<F>(t2247, t2251, t68, t26205, t6963, t45972, t7342, t10309, t26178, t25159, t2047, t92569);
        let (t95357, t95397, t95464, t95511, t95527, t95536) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1861::<F>(t116, t26209, t94973, t26375, t531, t198, t206, t7427, t2411, t26580, t25373, t26550);
    (t95310, t95314, t95316, t95319, t95320, t95340, t95357, t95397, t95464, t95511, t95527, t95536)
}
