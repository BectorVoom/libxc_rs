//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1812;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1813;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1814;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta463(t23253: f64, t6562: f64, t225: f64, t258: f64, t2710: f64, t214: f64, t1880: f64, t1883: f64, t23012: f64, t23237: f64, t6572: f64, t213: f64, t252: f64, t776: f64, t857: f64, t865: f64, t22986: f64, t6625: f64, t6576: f64, t10049: f64, t1912: f64, t23236: f64, t23239: f64, t23243: f64, t23250: f64, t23252: f64, t2597: f64, t2720: f64, t2743: f64, t6627: f64, t6663: f64, t866: f64, t9590: f64, t9593: f64, t23234: f64, t870: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23254, t23257, t23258, t23259, t23262, t23265, t23266, t23270) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1812(t23253, t6562, t225, t258, t2710, t214, t1880, t1883, t23012, t23237, t6572, t213, t252);
        let (t23272, t23273, t23278, t23281, t23284) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1813(t776, t857, t865, t23270, t22986, t225, t6625, t6576, t10049, t1912, t23236, t23239, t23243, t23250, t23252, t23254, t23259, t23262, t23266, t2597, t2720, t2743, t6627, t6663, t866, t9590, t9593);
        let (t23285, t23286) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1814(t23234, t23284, t870);
    (t23254, t23257, t23258, t23262, t23265, t23270, t23272, t23273, t23278, t23281, t23285, t23286)
}
