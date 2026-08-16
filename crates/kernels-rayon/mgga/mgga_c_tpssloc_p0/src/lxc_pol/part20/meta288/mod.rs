//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1490;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1491;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1492;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1493;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta288(t10768: f64, t10847: f64, t300: f64, t2940: f64, t2944: f64, t2924: f64, t2929: f64, t4497: f64, t959: f64, t10665: f64, t10699: f64, t10707: f64, t10711: f64, t10715: f64, t10729: f64, t10733: f64, t10739: f64, t10819: f64, t10658: f64, t360: f64, t1021: f64, t248: f64, t1004: f64, t3047: f64, t3053: f64, t3117: f64, t1043: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10849, t10851, t10853, t10855, t10856) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1490(t10768, t10847, t300, t2940, t2944, t2924, t2929, t4497, t959, t10665, t10699, t10707, t10711, t10715, t10729, t10733, t10739, t10819);
        let t10857 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1491(t10658, t10856);
        let (t10858, t10860, t10863) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1492(t10857, t360, t1021, t248, t1004, t3047);
        let (t10866, t10868) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1493(t3053, t3117, t1043, t676);
    (t10849, t10851, t10853, t10855, t10857, t10858, t10860, t10863, t10866, t10868)
}
