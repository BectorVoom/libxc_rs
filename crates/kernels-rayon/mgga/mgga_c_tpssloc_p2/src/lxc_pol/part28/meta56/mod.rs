//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta56 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk377;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk378;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk379;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk380;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk381;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta56(t636: f64, t607: f64, t1088: f64, t123: f64, t1087: f64, t423: f64, t419: f64, t409: f64, t410: f64, t1086: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1089 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk377(t636);
        let t1090 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk378(t1089, t607);
        let (t1091, t1092, t1094, t1096, t1097, t1098) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk379(t1088, t1090, t123, t1087, t423, t419);
        let t1099 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk380(t1098, t409);
        let t1100 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk381(t410);
        let t1102 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk382(t1086, t1092);
    (t1089, t1090, t1091, t1092, t1094, t1096, t1097, t1098, t1099, t1100, t1102)
}
