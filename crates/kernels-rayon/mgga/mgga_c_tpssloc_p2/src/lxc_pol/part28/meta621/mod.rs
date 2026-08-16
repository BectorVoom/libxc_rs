//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1942;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1943;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta621(t22779: f64, t26319: f64, t1358: f64, t26248: f64, t3862: f64, t7715: f64, t22705: f64, t22852: f64, t236: f64, t5286: f64, t550: f64, t26245: f64, t80791: f64, t22788: f64, t5310: f64, t16150: f64, t6952: f64, t16155: f64, t26271: f64, t80836: f64, t1361: f64, t22690: f64, t22792: f64, t5187: f64, t16148: f64, t26288: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91300, t91303, t91305, t91310, t91312) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1942(t22779, t26319, t1358, t26248, t3862, t7715, t22705, t22852, t236, t5286, t550, t26245, t80791);
        let (t91317, t91319, t91321, t91323, t91327, t91330) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1943(t22788, t5310, t16150, t6952, t16155, t26271, t80836, t1361, t22690, t22792, t5187, t16148, t26288);
    (t91300, t91303, t91305, t91310, t91312, t91317, t91319, t91321, t91323, t91327, t91330)
}
