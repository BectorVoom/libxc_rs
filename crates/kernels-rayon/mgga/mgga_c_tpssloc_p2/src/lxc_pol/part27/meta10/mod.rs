//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta10 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk72;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk73;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk74;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk75;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk76;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta10(t123: f64, t126: f64, t129: f64, t136: f64, t144: f64, t159: f64, t157: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t164, t167, t168, t172) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk72(t123, t126, t129, t136);
        let (t177, t180, t181) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk73(t123, t126, t129, t136);
        let t182 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk74(t172, t181);
        let t184 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk75(t144, t159, t168, t182);
        let t185 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk76(t157, t184);
    (t164, t167, t168, t172, t177, t180, t181, t182, t184, t185)
}
