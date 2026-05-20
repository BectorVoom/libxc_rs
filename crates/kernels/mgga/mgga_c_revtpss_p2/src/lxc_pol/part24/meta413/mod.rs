//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1355;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1356;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta413<F: Float>(t41306: F, t367: F, t371: F, t373: F, t9291: F, t2852: F, t3154: F, t11874: F, t15688: F, t11853: F, t828: F, t3181: F, t675: F, t283: F, t66: F, t11821: F, t41270: F, t11144: F, t3252: F, t11852: F, t126: F, t12166: F, t15905: F, t994: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42078, t42121, t42215, t42328, t42410, t42447) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1355::<F>(t41306, t367, t371, t373, t9291, t2852, t3154, t11874, t15688, t11853, t828, t3181, t675);
        let (t42472, t42508, t42518, t42534, t42621) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1356::<F>(t283, t2852, t66, t11821, t41270, t11144, t3252, t11852, t126, t12166, t15905, t994);
    (t42078, t42121, t42215, t42328, t42410, t42447, t42472, t42508, t42518, t42534, t42621)
}
