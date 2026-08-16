//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk554;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk555;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk556;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk557;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk558;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta99(t121: f64, t1229: f64, t374: f64, t486: f64, t677: f64, t485: f64, t221: f64, t3426: f64, t456: f64, t1176: f64, t3247: f64, t3242: f64, t3439: f64, t478: f64, t483: f64, t3068: f64, t1244: f64, t1230: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3521 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk554(t121, t1229);
        let t3540 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk555(t374, t486, t677);
        let (t3542, t3545, t3547, t3555, t3560, t3570) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk556(t3540, t485, t221, t3426, t456, t1176, t3247, t3242, t3439, t121, t486);
        let (t3575, t3576, t3577) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk557(t478, t483, t3068, t1244);
        let t3578 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk558(t1230, t820);
    (t3521, t3540, t3542, t3545, t3547, t3555, t3560, t3570, t3575, t3576, t3577, t3578)
}
