//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1326;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1327;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1328;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta364<F: Float>(t10623: F, t2952: F, t10632: F, t41825: F, t41827: F, t959: F, t10605: F, t2940: F, t41977: F, t942: F, t951: F, t41992: F, t41998: F, t42002: F, t42005: F, t42025: F, t42031: F, t42097: F, t42105: F, t10523: F, t300: F, t41764: F, t10853: F, t2925: F, t2951: F, t2929: F, t2932: F, t41733: F, t42110: F, t42113: F, t42145: F, t42148: F, t42233: F, t42235: F, t42238: F, t42241: F, t42661: F, t42679: F, t10510: F, t3114: F, t1020: F, t1021: F, t1023: F, t1025: F, t1041: F, t10426: F, t10433: F, t1046: F, t10463: F, t10863: F, t10876: F, t10952: F, t14164: F, t248: F, t3039: F, t3048: F, t3057: F, t3132: F, t360: F, t39097: F, t42468: F, t42622: F, t42624: F, t42639: F, t42648: F, t42651: F, t42653: F, t42658: F, t4582: F, t973: F, t974: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42682, t42686, t42688, t42692, t42693) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1326::<F>(t10623, t2952, t10632, t41825, t41827, t959, t10605, t2940, t41977, t942, t951, t41992, t41998, t42002, t42005, t42025, t42031, t42097, t42105);
        let (t42697, t42699, t42701, t42704, t42708) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1327::<F>(t10523, t41827, t951, t959, t300, t41764, t10853, t2940, t2925, t2951, t2929, t2932, t41733);
        let (t42712, t42713) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1328::<F>(t41827, t42110, t42113, t959, t42145, t42148, t42233, t42235, t42238, t42241, t42697, t42699, t42701, t42704, t42708);
        let (t42715, t42723) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1329::<F>(t42661, t42679, t42693, t42713, t10510, t3114, t1020, t1021, t1023, t1025, t1041, t10426, t10433, t1046, t10463, t10863, t10876, t10952, t14164, t248, t3039, t3048, t3057, t3132, t360, t39097, t42468, t42622, t42624, t42639, t42648, t42651, t42653, t42658, t4582, t973, t974);
    (t42682, t42686, t42688, t42692, t42697, t42699, t42701, t42704, t42708, t42712, t42715, t42723)
}
