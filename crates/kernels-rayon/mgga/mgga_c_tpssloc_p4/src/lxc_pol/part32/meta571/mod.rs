//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1945;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta571(t3: f64, t5398: f64, t1915: f64, t5527: f64, t1484: f64, t1530: f64, t1877: f64, t193: f64, t202: f64, t23295: f64, t2522: f64, t25358: f64, t28248: f64, t28447: f64, t4314: f64, t5544: f64, t5660: f64, t5664: f64, t6670: f64, t7541: f64, t870: f64, t28: f64, t23788: f64, t1649: f64, t22959: f64, t28448: f64, t5966: f64, t7649: f64, t7656: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28525, t28732, t28755) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1945(t3, t5398, t1915, t5527, t1484, t1530, t1877, t193, t202, t23295, t2522, t25358, t28248, t28447, t4314, t5544, t5660, t5664, t6670, t7541, t870);
        let (t28764, t28765, t28771, t28774, t28778, t28789, t28792, t28795, t28802) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1946(t28, t5527, t1915, t23788, t28248, t1484, t1649, t5544, t5664, t1530, t5660, t1877, t22959, t23295, t2522, t25358, t28448, t4314, t5966, t6670, t7541, t7649, t7656);
    (t28525, t28732, t28755, t28764, t28765, t28771, t28774, t28778, t28789, t28792, t28795, t28802)
}
