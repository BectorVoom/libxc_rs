//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta207 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk918;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk919;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk920;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta207(t252: f64, t5558: f64, t1492: f64, t1519: f64, t119: f64, t5527: f64, t210: f64, t5544: f64, t225: f64, t237: f64, t1509: f64, t2632: f64, t819: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5559, t5561, t5567, t5568, t5571, t5572, t5575) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk918(t252, t5558, t1492, t1519, t119, t5527, t210, t5544, t225);
        let (t5576, t5584) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk919(t237, t5575, t1509);
        let t5585 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk920(t2632, t5584);
        let t5587 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk921(t5585, t819, t820);
    (t5559, t5561, t5567, t5568, t5571, t5572, t5575, t5576, t5584, t5585, t5587)
}
