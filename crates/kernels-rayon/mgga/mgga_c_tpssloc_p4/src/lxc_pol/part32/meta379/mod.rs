//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1436;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1437;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1438;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1439;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1440;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta379(t3726: f64, t5227: f64, t3802: f64, t5234: f64, t3788: f64, t836: f64, t1336: f64, t5252: f64, t225: f64, t5319: f64, t5217: f64, t1390: f64, t5356: f64, t112: f64, t5363: f64, t111: f64, t1851: f64, t5392: f64, t9427: f64, t2433: f64, t5398: f64, t12603: f64, t12604: f64, t25: f64, t28: f64, zeta_threshold: f64, t40: f64, t52: f64, t3966: f64, t4080: f64, t607: f64, t73: f64, t9438: f64, t2440: f64, t4087: f64, t76: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16354, t16394, t16400, t16439, t16460, t16497) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1436(t3726, t5227, t3802, t5234, t3788, t836, t1336, t5252, t225, t5319, t5217, t1390, t5356);
        let (t16521, t16524) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1437(t112, t5363, t111, t1851);
        let (t16549, t16554, t16557) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1438(t5392, t9427, t2433, t5398, t12603, t12604);
        let t16558 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1439(t25, t28, t16557, zeta_threshold);
        let (t16562, t16574) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1440(t40, t52, t16549, t16554, t16558, t3966, t4080, t607, t73, t5392, t9438, t2440, t5398, t4087, t76, zeta_threshold);
    (t16354, t16394, t16400, t16439, t16460, t16497, t16521, t16524, t16557, t16558, t16562, t16574)
}
