//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1331;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1332;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1333;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1334;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta457(t75852: f64, t75862: f64, t75875: f64, t75891: f64, t75934: f64, t75947: f64, t76543: f64, t76556: f64, t41666: f64, t75836: f64, t123: f64, t41664: f64, t75912: f64, t883: f64, t882: f64, t41687: f64, t10564: f64, t17151: f64, t5398: f64, t10216: f64, t2768: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76559, t76572, t76574) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1331(t75852, t75862, t75875, t75891, t75934, t75947, t76543, t76556, t41666, t75836, t123, t41664);
        let (t76576, t76578) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1332(t75912, t883, t123, t882);
        let (t76581, t76583) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1333(t41687, t75836, t10564, t123);
        let (t76585, t76587) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1334(t17151, t5398, t10564, t123);
        let (t76589, t76591) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1335(t10216, t75836, t123, t2768);
    (t76559, t76572, t76574, t76576, t76578, t76581, t76583, t76585, t76587, t76589, t76591)
}
