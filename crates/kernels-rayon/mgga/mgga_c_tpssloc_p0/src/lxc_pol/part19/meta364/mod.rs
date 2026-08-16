//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1326;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1327;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1328;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta364(t10623: f64, t2952: f64, t10632: f64, t41825: f64, t41827: f64, t959: f64, t10605: f64, t2940: f64, t41977: f64, t942: f64, t951: f64, t41992: f64, t41998: f64, t42002: f64, t42005: f64, t42025: f64, t42031: f64, t42097: f64, t42105: f64, t10523: f64, t300: f64, t41764: f64, t10853: f64, t2925: f64, t2951: f64, t2929: f64, t2932: f64, t41733: f64, t42110: f64, t42113: f64, t42145: f64, t42148: f64, t42233: f64, t42235: f64, t42238: f64, t42241: f64, t42661: f64, t42679: f64, t10510: f64, t3114: f64, t1020: f64, t1021: f64, t1023: f64, t1025: f64, t1041: f64, t10426: f64, t10433: f64, t1046: f64, t10463: f64, t10863: f64, t10876: f64, t10952: f64, t14164: f64, t248: f64, t3039: f64, t3048: f64, t3057: f64, t3132: f64, t360: f64, t39097: f64, t42468: f64, t42622: f64, t42624: f64, t42639: f64, t42648: f64, t42651: f64, t42653: f64, t42658: f64, t4582: f64, t973: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42682, t42686, t42688, t42692, t42693) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1326(t10623, t2952, t10632, t41825, t41827, t959, t10605, t2940, t41977, t942, t951, t41992, t41998, t42002, t42005, t42025, t42031, t42097, t42105);
        let (t42697, t42699, t42701, t42704, t42708) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1327(t10523, t41827, t951, t959, t300, t41764, t10853, t2940, t2925, t2951, t2929, t2932, t41733);
        let (t42712, t42713) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1328(t41827, t42110, t42113, t959, t42145, t42148, t42233, t42235, t42238, t42241, t42697, t42699, t42701, t42704, t42708);
        let (t42715, t42723) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1329(t42661, t42679, t42693, t42713, t10510, t3114, t1020, t1021, t1023, t1025, t1041, t10426, t10433, t1046, t10463, t10863, t10876, t10952, t14164, t248, t3039, t3048, t3057, t3132, t360, t39097, t42468, t42622, t42624, t42639, t42648, t42651, t42653, t42658, t4582, t973, t974);
    (t42682, t42686, t42688, t42692, t42697, t42699, t42701, t42704, t42708, t42712, t42715, t42723)
}
