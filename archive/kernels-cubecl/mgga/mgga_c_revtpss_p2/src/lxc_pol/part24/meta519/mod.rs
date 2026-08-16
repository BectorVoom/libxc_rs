//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1543;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1544;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1545;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1546;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta519<F: Float>(t24233: F, t689: F, t24241: F, t24249: F, t24407: F, t3520: F, t24294: F, t698: F, t24288: F, t24291: F, t24274: F, t24271: F, t24312: F, t3390: F, t24297: F, t24323: F, t3435: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t81232 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1543::<F>(t24233, t689);
        let t81234 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1544::<F>(t24241, t689);
        let t81236 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1545::<F>(t24249, t689);
        let (t81310, t81425, t81427, t81429, t81491, t81496, t81513, t81539, t81650) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1546::<F>(t24407, t3520, t24294, t698, t24288, t24291, t24274, t24271, t24312, t3390, t24297, t24323, t3435);
    (t81232, t81234, t81236, t81310, t81425, t81427, t81429, t81491, t81496, t81513, t81539, t81650)
}
