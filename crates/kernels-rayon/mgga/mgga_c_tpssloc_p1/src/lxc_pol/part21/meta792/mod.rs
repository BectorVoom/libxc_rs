//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta792 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2752;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta792(t46244: f64, t185: f64, t2658: f64, t55723: f64, t152: f64, t2244: f64, t5499: f64, t4303: f64, t868: f64, t12892: f64, t16693: f64, t16616: f64, t2535: f64, t46278: f64, t10126: f64, t12895: f64, t12915: f64, t1484: f64, t16662: f64, t1877: f64, t2522: f64, t2523: f64, t39483: f64, t4255: f64, t4314: f64, t46213: f64, t5527: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t57996, t58005, t58008, t58009, t58020, t58021) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2752(t46244, t185, t2658, t55723, t152, t2244, t5499, t4303, t868, t12892, t16693, t16616, t2535);
        let (t58022, t58023, t58024) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2753(t58021, t46278, t10126, t12895, t12915, t1484, t16662, t1877, t2522, t2523, t39483, t4255, t4314, t46213, t5527, t57996, t58005, t58008, t58009, t58020);
    (t57996, t58005, t58008, t58020, t58022, t58023, t58024)
}
