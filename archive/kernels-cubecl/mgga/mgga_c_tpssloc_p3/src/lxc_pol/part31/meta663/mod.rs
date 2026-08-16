//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1951;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1952;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta663<F: Float>(t1408: F, t4255: F, t870: F, t25365: F, t57911: F, t10143: F, t1484: F, t25374: F, t23788: F, t67128: F, t16949: F, t25891: F, t25927: F, t98102: F, t5966: F, t868: F, t1649: F, t28248: F, t83555: F, t98030: F, t98011: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t99060, t100562, t100572, t100638, t100641) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1951::<F>(t1408, t4255, t870, t25365, t57911, t10143, t1484, t25374, t23788, t67128, t16949, t25891);
        let (t100644, t100646, t100651, t100656, t100659, t100664) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1952::<F>(t25927, t98102, t5966, t868, t1649, t4255, t870, t28248, t83555, t98030, t23788, t98011);
    (t99060, t100562, t100572, t100638, t100641, t100644, t100646, t100651, t100656, t100659, t100664)
}
