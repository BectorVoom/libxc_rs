//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2176;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2177;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2178;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta531<F: Float>(t1196: F, t16682: F, t12500: F, t5205: F, t1733: F, t3385: F, t3433: F, t3302: F, t5332: F, t1214: F, t5333: F, t1716: F, t2435: F, t5048: F, t689: F, t5053: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16684, t16685, t16687, t16688, t16690, t16695) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2176::<F>(t1196, t16682, t12500, t5205, t1733, t3385, t3433, t3302, t5332);
        let (t16696, t16697, t16706) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2177::<F>(t1214, t5333, t16695, t1716, t2435);
        let t16708 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2178::<F>(t5048, t689);
        let t16710 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2179::<F>(t5053, t689);
    (t16684, t16685, t16687, t16688, t16690, t16695, t16696, t16697, t16706, t16708, t16710)
}
