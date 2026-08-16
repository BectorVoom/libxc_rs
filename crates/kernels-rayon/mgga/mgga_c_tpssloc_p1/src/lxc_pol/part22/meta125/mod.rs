//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta125 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk841;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk842;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk843;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk844;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk845;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk846;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk847;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk848;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta125(t1329: f64, t3726: f64, t1332: f64, t68: f64, t1340: f64, t1333: f64, t1358: f64, t1362: f64, t1337: f64, t551: f64, t236: f64, t240: f64, t1336: f64, t550: f64, t1339: f64, t835: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3763, t3777) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk841(t1329, t3726, t1332, t68);
        let t3778 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk842(t1340, t3777);
        let (t3781, t3783) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk843(t1333, t1358, t1362, t3777);
        let t3787 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk844(t1337, t551);
        let t3788 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk845(t236, t3787);
        let (t3789, t3790, t3792) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk846(t240, t3788, t1336, t550);
        let t3798 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk847(t1339, t835);
        let t3799 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk848(t1336, t3798);
    (t3763, t3777, t3778, t3781, t3783, t3787, t3788, t3789, t3790, t3792, t3798, t3799)
}
