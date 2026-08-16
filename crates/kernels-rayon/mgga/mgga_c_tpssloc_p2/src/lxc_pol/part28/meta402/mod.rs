//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1560;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta402(t2632: f64, t4233: f64, t1441: f64, t671: f64, t1388: f64, t1799: f64, t3792: f64, t5286: f64, t576: f64, t107: f64, t240: f64, t625: f64, t656: f64, t666: f64, t2331: f64, t63: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16935, t19456) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1560(t2632, t4233, t1441, t671);
        let (t19577, t19735, t20173, t22468, t22470, t22471, t22472, t22473) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1561(t1388, t1799, t3792, t5286, t576, t671, t107, t240, t625, t656, t666, t2331, t63);
    (t16935, t19456, t19577, t19735, t20173, t22468, t22470, t22471, t22472, t22473)
}
