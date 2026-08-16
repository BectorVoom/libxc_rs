//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta103 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk570;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk571;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk572;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk573;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk574;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta103(t236: f64, t3787: f64, t240: f64, t1336: f64, t550: f64, t1339: f64, t835: f64, t242: f64, t1365: f64, t67: f64, t246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3788 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk570(t236, t3787);
        let (t3789, t3790, t3792) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk571(t240, t3788, t1336, t550);
        let (t3798, t3799) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk572(t1339, t835, t1336);
        let (t3802, t3803) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk573(t1339, t242, t1336);
        let (t3804, t3805) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk574(t1365, t67, t246);
    (t3788, t3789, t3790, t3792, t3798, t3799, t3802, t3803, t3804, t3805)
}
