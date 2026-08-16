//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2176;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2177;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2178;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta531(t1196: f64, t16682: f64, t12500: f64, t5205: f64, t1733: f64, t3385: f64, t3433: f64, t3302: f64, t5332: f64, t1214: f64, t5333: f64, t1716: f64, t2435: f64, t5048: f64, t689: f64, t5053: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16684, t16685, t16687, t16688, t16690, t16695) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2176(t1196, t16682, t12500, t5205, t1733, t3385, t3433, t3302, t5332);
        let (t16696, t16697, t16706) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2177(t1214, t5333, t16695, t1716, t2435);
        let t16708 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2178(t5048, t689);
        let t16710 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2179(t5053, t689);
    (t16684, t16685, t16687, t16688, t16690, t16695, t16696, t16697, t16706, t16708, t16710)
}
