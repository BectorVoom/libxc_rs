//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta685 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2256;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2257;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2258;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2259;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2260;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2261;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2262;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta685(t18242: f64, t690: f64, t2394: f64, t5976: f64, t18216: f64, t18212: f64, t18226: f64, t18222: f64, t3375: f64, t6063: f64, t18893: f64, t3359: f64, t11285: f64, t6084: f64, t18785: f64, t3403: f64, t18834: f64, t3315: f64, t1147: f64, t18710: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t63336 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2256(t18242, t690);
        let t63361 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2257(t2394, t5976);
        let t63382 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2258(t18216, t690);
        let t63384 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2259(t18212, t690);
        let t63398 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2260(t18226, t690);
        let t63400 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2261(t18222, t690);
        let (t63454, t63502, t63519, t63533, t63588, t63597) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2262(t3375, t6063, t18893, t3359, t11285, t6084, t18785, t3403, t18834, t3315, t1147, t18710);
    (t63336, t63361, t63382, t63384, t63398, t63400, t63454, t63502, t63519, t63533, t63588, t63597)
}
