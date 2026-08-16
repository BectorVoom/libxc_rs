//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta159 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk832;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk833;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk834;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta159(t4182: f64, t4282: f64, t1510: f64, t2732: f64, t4234: f64, t860: f64, t68: f64, t814: f64, t226: f64, t829: f64, t1519: f64, t235: f64, t4265: f64, t1499: f64, t1523: f64, t1525: f64, t255: f64, t2617: f64, t4162: f64, t4166: f64, t4281: f64, t808: f64, t812: f64, t861: f64, t863: f64, t858: f64, t1528: f64, t259: f64, t2597: f64, t2713: f64, t4143: f64, t4145: f64, t4147: f64, t4149: f64, t4266: f64, t4268: f64, t4273: f64, t855: f64, t866: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4283, t4286, t4288, t4290, t4291) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk832(t4182, t4282, t1510, t2732, t4234, t860, t68, t814, t226);
        let (t4292, t4295, t4296, t4298, t4300) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk833(t4282, t829, t1519, t814, t235, t4265, t1499, t1523, t1525, t226, t255, t2617, t4162, t4166, t4281, t4283, t4286, t4288, t4291, t808, t812, t861, t863);
        let t4301 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk834(t4300, t858);
        let t4303 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk835(t1528, t259, t2597, t2713, t4143, t4145, t4147, t4149, t4266, t4268, t4273, t4301, t855, t866);
    (t4283, t4286, t4288, t4290, t4291, t4292, t4295, t4296, t4298, t4300, t4301, t4303)
}
