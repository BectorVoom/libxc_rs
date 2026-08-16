//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta880 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3050;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3051;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3052;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta880(t10523: f64, t14606: f64, t1568: f64, t2722: f64, t2723: f64, t2782: f64, t4503: f64, t10661: f64, t14602: f64, t1558: f64, t2482: f64, t10535: f64, t14523: f64, t9285: f64, t10073: f64, t14496: f64, t231: f64, t2783: f64, t14946: f64, t2710: f64, t4469: f64, t836: f64, t14598: f64, t14600: f64, t2434: f64, t10111: f64, t22: f64, t4518: f64, t10871: f64, t10952: f64, t122: f64, t676: f64, t72: f64, t51306: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51623, t51625, t51628, t51632, t51635) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3050(t10523, t14606, t1568, t2722, t2723, t2782, t4503, t10661, t14602, t1558, t2482, t10535, t14523, t9285);
        let (t51637, t51642, t51646, t51653, t51657) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3051(t10073, t14496, t231, t2782, t2783, t51625, t14946, t2710, t9285, t4469, t836, t14598, t14600, t2434);
        let (t51660, t51668, t51672) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3052(t10111, t22, t4518, t10871, t10952, t122, t1558, t2482, t2722, t676, t72, t231, t2782, t2783, t51306);
    (t51623, t51628, t51632, t51635, t51637, t51642, t51646, t51653, t51657, t51660, t51668, t51672)
}
