//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1148;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1149;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta353<F: Float>(t23076: F, t241: F, t67: F, t2559: F, t2570: F, t782: F, t9558: F, t786: F, t9569: F, t222: F, t39934: F, t2691: F, t812: F, t815: F, t238: F, t244: F, t248: F, t40445: F, t116: F, t207: F, t40419: F, t9538: F, t154: F, t1891: F, t205: F, t792: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40971, t41008, t41011, t41083, t41096, t41115) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1148::<F>(t23076, t241, t67, t2559, t2570, t782, t9558, t786, t9569, t222, t39934, t2691, t812, t815);
        let (t41139, t41146, t41155, t41161, t41170) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1149::<F>(t238, t244, t248, t40445, t116, t207, t40419, t9538, t154, t1891, t205, t792, t9558);
    (t40971, t41008, t41011, t41083, t41096, t41115, t41139, t41146, t41155, t41161, t41170)
}
