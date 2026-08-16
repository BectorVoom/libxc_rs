//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1877;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta586(t22690: f64, t23171: f64, t25319: f64, t1888: f64, t22996: f64, t2632: f64, t87106: f64, t23143: f64, t7525: f64, t25238: f64, t6579: f64, t23153: f64, t4119: f64, t6552: f64, t6637: f64, t12971: f64, t6638: f64, t22893: f64, t23164: f64, t25312: f64, t232: f64, t47425: f64, t6646: f64, t25038: f64, t25248: f64, t776: f64, t87130: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87653, t87660, t87666, t87668, t87672) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1877(t22690, t23171, t25319, t1888, t22996, t2632, t87106, t23143, t7525, t25238, t6579, t23153, t4119, t6552, t6637);
        let (t87676, t87679, t87692, t87699) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1878(t12971, t6552, t6637, t6638, t22893, t23164, t25312, t1888, t232, t47425, t6646, t25038, t25248, t776, t87130);
    (t87653, t87660, t87666, t87668, t87672, t87676, t87679, t87692, t87699)
}
