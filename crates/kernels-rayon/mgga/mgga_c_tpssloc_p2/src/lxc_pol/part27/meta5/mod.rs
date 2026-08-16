//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta5 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk36;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk37;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk38;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk39;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk40;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk41;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta5(t40: f64, t73: f64, t52: f64, t72: f64, t66: f64, t5: f64, t24: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74, t75, t76) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk36(t40, t73, t52);
        let (t77, t78, t79) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk37(t52, t76, t75);
        let t80 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk38(t72, t79);
        let (t83, t84, t85) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk39(t66, t80);
        let t86 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk40(t85);
        let t88 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk41(t5, t24, t86);
    (t74, t75, t76, t77, t78, t79, t80, t83, t84, t85, t86, t88)
}
