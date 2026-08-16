//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1945;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta571<F: Float>(t3: F, t5398: F, t1915: F, t5527: F, t1484: F, t1530: F, t1877: F, t193: F, t202: F, t23295: F, t2522: F, t25358: F, t28248: F, t28447: F, t4314: F, t5544: F, t5660: F, t5664: F, t6670: F, t7541: F, t870: F, t28: F, t23788: F, t1649: F, t22959: F, t28448: F, t5966: F, t7649: F, t7656: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t28525, t28732, t28755) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1945::<F>(t3, t5398, t1915, t5527, t1484, t1530, t1877, t193, t202, t23295, t2522, t25358, t28248, t28447, t4314, t5544, t5660, t5664, t6670, t7541, t870);
        let (t28764, t28765, t28771, t28774, t28778, t28789, t28792, t28795, t28802) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1946::<F>(t28, t5527, t1915, t23788, t28248, t1484, t1649, t5544, t5664, t1530, t5660, t1877, t22959, t23295, t2522, t25358, t28448, t4314, t5966, t6670, t7541, t7649, t7656);
    (t28525, t28732, t28755, t28764, t28765, t28771, t28774, t28778, t28789, t28792, t28795, t28802)
}
