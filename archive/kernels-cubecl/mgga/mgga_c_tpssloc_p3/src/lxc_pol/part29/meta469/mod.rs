//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1799;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1800;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1801;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta469<F: Float>(t23253: F, t6562: F, t225: F, t258: F, t2710: F, t214: F, t1880: F, t1883: F, t23012: F, t23237: F, t6572: F, t213: F, t252: F, t776: F, t857: F, t865: F, t22986: F, t6625: F, t6576: F, t10049: F, t1912: F, t23236: F, t23239: F, t23243: F, t23250: F, t23252: F, t2597: F, t2720: F, t2743: F, t6627: F, t6663: F, t866: F, t9590: F, t9593: F, t23234: F, t870: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23254, t23257, t23258, t23259, t23262, t23265, t23266, t23270) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1799::<F>(t23253, t6562, t225, t258, t2710, t214, t1880, t1883, t23012, t23237, t6572, t213, t252);
        let (t23272, t23273, t23278, t23281, t23284) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1800::<F>(t776, t857, t865, t23270, t22986, t225, t6625, t6576, t10049, t1912, t23236, t23239, t23243, t23250, t23252, t23254, t23259, t23262, t23266, t2597, t2720, t2743, t6627, t6663, t866, t9590, t9593);
        let (t23285, t23286) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1801::<F>(t23234, t23284, t870);
    (t23254, t23257, t23258, t23262, t23265, t23270, t23272, t23273, t23278, t23281, t23285, t23286)
}
