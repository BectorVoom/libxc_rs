//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1535;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1536;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1537;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1538;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1539;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta421(t154: f64, t835: f64, t3748: f64, t212: f64, t562: f64, t6890: f64, t1372: f64, t214: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t22641 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1535(t154, t835);
        let t22642 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1536(t22641, t3748);
        let t22643 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1537(t212, t562);
        let (t22644, t22645, t22666) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1538(t22643, t6890, t22642, t1372, t214);
        let t22674 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1539(t562, t794);
    (t22641, t22642, t22643, t22644, t22645, t22666, t22674)
}
