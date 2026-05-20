//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1418;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta388<F: Float>(t11328: F, t698: F, t2439: F, t2915: F, t2909: F, t11345: F, t41246: F, t41250: F, t41255: F, t41260: F, t41265: F, t41267: F, t41273: F, t41275: F, t41279: F, t41281: F, t11342: F, t11821: F, t240: F, t2851: F, t39443: F, t141: F, t39457: F, t905: F, t930: F, t25273: F, t268: F, t271: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41283, t41285, t41287, t41289, t41291) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1418::<F>(t11328, t698, t2439, t2915, t2909, t11345, t41246, t41250, t41255, t41260, t41265, t41267, t41273, t41275, t41279, t41281);
        let (t41292, t41296, t41297, t41299, t41301, t41303, t41306) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1419::<F>(t11342, t698, t11821, t240, t2851, t39443, t141, t39457, t905, t930, t25273, t268, t271);
    (t41283, t41285, t41287, t41289, t41291, t41292, t41296, t41297, t41299, t41301, t41303, t41306)
}
