//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta669 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2097;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta669(t5303: f64, t80820: f64, t22783: f64, t5310: f64, t1827: f64, t80914: f64, t1811: f64, t80775: f64, t7709: f64, t80766: f64, t22797: f64, t5227: f64, t22804: f64, t26277: f64, t225: f64, t26221: f64, t22674: f64, t22892: f64, t26189: f64, t26329: f64, t26229: f64, t22724: f64, t26344: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91365, t91387, t91394, t91398, t91400, t91402) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2097(t5303, t80820, t22783, t5310, t1827, t80914, t1811, t80775, t7709, t80766, t22797, t5227);
        let (t91403, t91404, t91441, t91487, t91488, t91491, t91531) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2098(t91402, t22804, t26277, t225, t26221, t22674, t22892, t26189, t26329, t26229, t22724, t26344);
    (t91365, t91387, t91394, t91398, t91400, t91403, t91404, t91441, t91487, t91488, t91491, t91531)
}
