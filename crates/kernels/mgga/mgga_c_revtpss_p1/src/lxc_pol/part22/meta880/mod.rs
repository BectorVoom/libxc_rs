//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta880 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3050;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3051;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3052;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta880<F: Float>(t10523: F, t14606: F, t1568: F, t2722: F, t2723: F, t2782: F, t4503: F, t10661: F, t14602: F, t1558: F, t2482: F, t10535: F, t14523: F, t9285: F, t10073: F, t14496: F, t231: F, t2783: F, t14946: F, t2710: F, t4469: F, t836: F, t14598: F, t14600: F, t2434: F, t10111: F, t22: F, t4518: F, t10871: F, t10952: F, t122: F, t676: F, t72: F, t51306: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t51623, t51625, t51628, t51632, t51635) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3050::<F>(t10523, t14606, t1568, t2722, t2723, t2782, t4503, t10661, t14602, t1558, t2482, t10535, t14523, t9285);
        let (t51637, t51642, t51646, t51653, t51657) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3051::<F>(t10073, t14496, t231, t2782, t2783, t51625, t14946, t2710, t9285, t4469, t836, t14598, t14600, t2434);
        let (t51660, t51668, t51672) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3052::<F>(t10111, t22, t4518, t10871, t10952, t122, t1558, t2482, t2722, t676, t72, t231, t2782, t2783, t51306);
    (t51623, t51628, t51632, t51635, t51637, t51642, t51646, t51653, t51657, t51660, t51668, t51672)
}
