//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1452;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1453;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta472(t14524: f64, t51297: f64, t136: f64, t2457: f64, t39680: f64, t6022: f64, t10073: f64, t18746: f64, t18742: f64, t10069: f64, t2718: f64, t6041: f64, t18729: f64, t2470: f64, t2798: f64, t2482: f64, t6016: f64, t879: f64, t14563: f64, t14568: f64, t10535: f64, t6017: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62874, t62907, t62909, t62920, t62922, t62929) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1452(t14524, t51297, t136, t2457, t39680, t6022, t10073, t18746, t18742, t10069, t2718, t6041);
        let (t62952, t62967, t62983, t62999) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1453(t18729, t2470, t2798, t2482, t6016, t879, t14563, t14568, t10535, t136, t2457, t6017);
    (t62874, t62907, t62909, t62920, t62922, t62929, t62952, t62967, t62983, t62999)
}
