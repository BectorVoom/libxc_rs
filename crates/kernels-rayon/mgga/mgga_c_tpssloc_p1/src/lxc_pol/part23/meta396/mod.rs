//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta396 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1201;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1202;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta396(t5572: f64, t9541: f64, t5624: f64, t9601: f64, t1512: f64, t47092: f64, t16673: f64, t2642: f64, t5614: f64, t9671: f64, t41008: f64, t5568: f64, t41385: f64, t5587: f64, t2629: f64, t2696: f64, t118: f64, t2375: f64, t5522: f64, t16710: f64, t2663: f64, t2517: f64, t2658: f64, t5392: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58550, t58574, t58576, t58642, t58723, t58744) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1201(t5572, t9541, t5624, t9601, t1512, t47092, t16673, t2642, t5614, t9671, t41008, t5568);
        let (t58809, t58811, t58844, t58972, t58984, t59013) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1202(t41385, t5587, t16673, t2629, t2696, t118, t2375, t5522, t16710, t2663, t2517, t2658, t5392);
    (t58550, t58574, t58576, t58642, t58723, t58744, t58809, t58811, t58844, t58972, t58984, t59013)
}
