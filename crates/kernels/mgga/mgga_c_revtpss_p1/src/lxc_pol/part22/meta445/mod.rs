//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2088;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta445<F: Float>(t1580: F, t2440: F, t2439: F, t1569: F, t2453: F, t2458: F, t10503: F, t10507: F, t10511: F, t10984: F, t10987: F, t14998: F, t15004: F, t15006: F, t15010: F, t15011: F, t2829: F, t4474: F, t887: F, t4533: F, t886: F, t2770: F, t1579: F, t2828: F, t10989: F, t10992: F, t10998: F, t11000: F, t11004: F, t11013: F, t11017: F, t11019: F, t11022: F, t2765: F, t4487: F, t4534: F, t865: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15014, t15015, t15017, t15018, t15022) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2088::<F>(t1580, t2440, t2439, t1569, t2453, t2458, t10503, t10507, t10511, t10984, t10987, t14998, t15004, t15006, t15010, t15011, t2829, t4474, t887);
        let (t15029, t15030, t15038, t15044) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2089::<F>(t4533, t886, t2770, t1579, t2828, t10989, t10992, t10998, t11000, t11004, t11013, t11017, t11019, t11022, t2765, t4487, t4534, t865);
    (t15014, t15015, t15017, t15018, t15022, t15029, t15030, t15038, t15044)
}
