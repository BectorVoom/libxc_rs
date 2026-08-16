//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta701 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2197;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta701<F: Float>(t2020: F, t97804: F, t15868: F, t1983: F, t7753: F, t22574: F, t74032: F, t8643: F, t28237: F, t532: F, t6879: F, t510: F, t652: F, t96729: F, t1874: F, t96683: F, t25992: F, t7685: F, t25985: F, t28821: F, t7000: F, t24990: F, t26167: F, t7687: F, t91620: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t97805, t97808, t97811, t97820, t97829) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2197::<F>(t2020, t97804, t15868, t1983, t7753, t22574, t74032, t8643, t28237, t532, t6879, t510, t652, t96729);
        let (t97831, t97833, t97835, t97836, t97839, t97842) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2198::<F>(t1874, t96683, t25992, t7685, t25985, t28821, t7000, t1983, t24990, t26167, t7687, t91620);
    (t97805, t97808, t97811, t97820, t97829, t97831, t97833, t97835, t97836, t97839, t97842)
}
