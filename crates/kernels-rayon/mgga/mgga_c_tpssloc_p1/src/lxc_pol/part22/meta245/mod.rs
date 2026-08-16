//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta245 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1348;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1349;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1350;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1351;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1352;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta245(t1013: f64, t363: f64, t3034: f64, t6793: f64, t368: f64, t10472: f64, t3131: f64, t360: f64, t376: f64, t676: f64, t1023: f64, t248: f64, t1020: f64, t2928: f64, t320: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10473, t10474, t10475, t10477) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1348(t1013, t363, t3034, t6793);
        let (t10478, t10479, t10480) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1349(t10477, t368, t10475, t10472);
        let t10482 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1350(t3131, t360);
        let t10508 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1351(t376, t676);
        let (t10510, t10511, t10523) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1352(t1023, t10508, t248, t1020, t2928, t320);
    (t10473, t10474, t10475, t10477, t10478, t10479, t10480, t10482, t10508, t10510, t10511, t10523)
}
