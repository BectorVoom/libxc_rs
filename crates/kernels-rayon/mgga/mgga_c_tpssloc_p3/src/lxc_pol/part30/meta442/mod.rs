//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1692;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1693;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta442(t212: f64, t562: f64, t6890: f64, t22642: f64, t225: f64, t6911: f64, t1372: f64, t214: f64, t6956: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t22643 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1692(t212, t562);
        let (t22644, t22646, t22656, t22666) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1693(t22643, t6890, t22642, t225, t6911, t1372, t214);
        let (t22670, t22674) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1694(t225, t6956, t562, t794);
    (t22643, t22644, t22646, t22656, t22666, t22670, t22674)
}
