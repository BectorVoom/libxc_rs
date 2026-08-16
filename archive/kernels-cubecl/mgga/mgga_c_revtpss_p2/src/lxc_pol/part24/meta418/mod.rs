//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1365;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1366;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta418<F: Float>(t342: F, t43471: F, t3154: F, t43351: F, t16551: F, t994: F, t16558: F, t11627: F, t42859: F, t11631: F, t3494: F, t3519: F, t13026: F, t240: F, t3361: F, t2304: F, t25273: F, t268: F, t404: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t43472, t43473, t43520, t43524, t43537, t43538, t43752) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1365::<F>(t342, t43471, t3154, t43351, t16551, t994, t16558, t11627, t42859, t11631, t3494, t3519);
        let (t43764, t43766, t43776, t43813) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1366::<F>(t13026, t240, t3361, t2304, t25273, t268, t404);
    (t43472, t43473, t43520, t43524, t43537, t43538, t43752, t43764, t43766, t43776, t43813)
}
