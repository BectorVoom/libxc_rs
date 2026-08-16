//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta258 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1199;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1200;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1201;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1202;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1203;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta258(t50: f64, t55: f64, t607: f64, t6503: f64, t67: f64, t1864: f64, t2109: f64, t6509: f64, t5: f64, t1860: f64, t2110: f64, t6486: f64, t6492: f64, t6495: f64, t7246: f64, t112: f64, t111: f64, t2113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7251 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1199(t50, t55);
        let (t7254, t7255) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1200(t607, t6503, t7251, t67);
        let t7256 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1201(t1864, t7255);
        let t7259 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1202(t2109, t6509);
        let (t7263, t7264) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1203(t5, t1860, t2110, t6486, t6492, t6495, t7246, t7256, t7259, t112);
        let t7266 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1204(t111, t2113);
    (t7251, t7254, t7255, t7256, t7259, t7263, t7264, t7266)
}
