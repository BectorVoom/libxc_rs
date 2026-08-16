//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1215;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1216;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1217;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta404(t3131: f64, t5866: f64, t3199: f64, t61734: f64, t3185: f64, t2394: f64, t5972: f64, t5980: f64, t5976: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t62840, t63004, t63183, t63332) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1215(t3131, t5866, t3199, t61734, t3185, t2394, t5972);
        let t63334 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1216(t2394, t5980);
        let t63361 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1217(t2394, t5976);
    (t62840, t63004, t63183, t63332, t63334, t63361)
}
