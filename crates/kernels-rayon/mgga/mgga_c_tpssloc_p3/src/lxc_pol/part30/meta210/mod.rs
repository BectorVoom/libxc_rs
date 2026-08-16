//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta210 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk987;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk988;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk989;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk990;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk991;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta210(t237: f64, t5575: f64, t1509: f64, t2632: f64, t819: f64, t820: f64, t1484: f64, t232: f64, t2645: f64, t4181: f64, t4212: f64, t185: f64, t5398: f64, t707: f64, t2373: f64, t2377: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t2486: f64, t2518: f64, t2530: f64, t2537: f64, t2665: f64, t5497: f64, t5498: f64, t5501: f64, t5506: f64, t5521: f64, t5524: f64, t5525: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5576, t5584) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk987(t237, t5575, t1509);
        let t5585 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk988(t2632, t5584);
        let t5587 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk989(t5585, t819, t820);
        let (t5591, t5593) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk990(t1484, t232, t2645, t4181);
        let (t5596, t5597, t5599, t5600) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk991(t4212, t185, t5398, t707, t2373, t2377, t2408, t2417, t2423, t2426, t2486, t2518, t2530, t2537, t2665, t5497, t5498, t5501, t5506, t5521, t5524, t5525);
    (t5576, t5584, t5585, t5587, t5591, t5593, t5596, t5597, t5599, t5600)
}
