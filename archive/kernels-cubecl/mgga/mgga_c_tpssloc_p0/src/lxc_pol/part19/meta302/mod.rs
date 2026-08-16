//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1088;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta302<F: Float>(t3791: F, t562: F, t10: F, t2229: F, t116: F, t117: F, t556: F, t252: F, t2631: F, t243: F, t828: F, t852: F, t3034: F, t371: F) -> (F, F, F, F, F, F, F, F) {
        let (t22740, t22811, t22815, t22843, t22997, t23076, t23175) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1088::<F>(t3791, t562, t10, t2229, t116, t117, t556, t252, t2631, t243, t828, t852);
        let t23508 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1089::<F>(t3034, t371);
    (t22740, t22811, t22815, t22843, t22997, t23076, t23175, t23508)
}
