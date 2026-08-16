//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2055;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2056;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta555<F: Float>(t2559: F, t2570: F, t782: F, t9558: F, t2617: F, t9600: F, t786: F, t9569: F, t805: F, t222: F, t39934: F, t9637: F, t2691: F, t812: F, t815: F, t10024: F, t809: F, t238: F, t244: F, t248: F, t40445: F, t9525: F, t9577: F, t116: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41008, t41011, t41052, t41083, t41084, t41096, t41107) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2055::<F>(t2559, t2570, t782, t9558, t2617, t9600, t786, t9569, t805, t222, t39934, t9637);
        let (t41115, t41130, t41139, t41144, t41146) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2056::<F>(t2691, t812, t815, t10024, t809, t238, t244, t248, t40445, t9525, t9577, t116);
    (t41008, t41011, t41052, t41083, t41084, t41096, t41107, t41115, t41130, t41139, t41144, t41146)
}
