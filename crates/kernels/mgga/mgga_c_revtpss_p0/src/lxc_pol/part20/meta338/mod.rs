//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1262;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta338<F: Float>(t136: F, t243: F, t220: F, t10769: F, t828: F, t2746: F, t240: F, t849: F, t10868: F, t241: F, t820: F, t231: F, t2394: F, t2719: F, t844: F, t2482: F, t814: F, t11509: F, t2988: F, t4900: F, t999: F, t4894: F, t245: F, t4890: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14686, t14785, t14791, t14832, t14894, t14917) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1262::<F>(t136, t243, t220, t10769, t828, t2746, t240, t849, t10868, t241, t820, t231, t2394);
        let (t14923, t14931, t15542, t15604, t15609, t15687) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1263::<F>(t2719, t820, t844, t2482, t814, t11509, t2988, t4900, t999, t4894, t245, t4890);
    (t14686, t14785, t14791, t14832, t14894, t14917, t14923, t14931, t15542, t15604, t15609, t15687)
}
