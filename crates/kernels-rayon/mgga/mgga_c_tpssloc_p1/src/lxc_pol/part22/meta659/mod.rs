//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta659 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2201;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2202;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta659(t13261: f64, t4166: f64, t118: f64, t2375: f64, t5522: f64, t16575: f64, t706: f64, t16710: f64, t2663: f64, t157: f64, t46387: f64, t12939: f64, t5392: f64, t607: f64, t750: f64, t2517: f64, t2658: f64, t12923: f64, t3966: f64, t4194: f64, t12924: f64, t16693: f64, t16616: f64, t2528: f64, t12932: f64, t4205: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58904, t58972, t58976, t58984, t58994, t59004) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2201(t13261, t4166, t118, t2375, t5522, t16575, t706, t16710, t2663, t157, t46387, t12939, t5392, t607, t750);
        let (t59013, t59022, t59024, t59028, t59032) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2202(t2517, t2658, t5392, t12923, t3966, t4194, t12924, t16693, t16616, t2528, t12932, t4205);
    (t58904, t58972, t58976, t58984, t58994, t59004, t59013, t59022, t59024, t59028, t59032)
}
