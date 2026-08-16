//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta651 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1928;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1929;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1930;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta651(t1888: f64, t232: f64, t58166: f64, t6646: f64, t16815: f64, t22986: f64, t2647: f64, t58226: f64, t23110: f64, t23185: f64, t28418: f64, t59331: f64, t23168: f64, t28330: f64, t5631: f64, t828: f64, t25319: f64, t4119: f64, t6552: f64, t6637: f64, t234: f64, t776: f64, t16758: f64, t5593: f64, t81865: f64, t16924: f64, t23146: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98530, t98534, t98546, t98549, t98553) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1928(t1888, t232, t58166, t6646, t16815, t22986, t2647, t58226, t23110, t23185, t28418, t59331);
        let (t98564, t98571, t98575, t98598) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1929(t23168, t28330, t1888, t232, t5631, t6646, t828, t25319, t4119, t6552, t6637, t234);
        let (t98601, t98608, t98610, t98612) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1930(t6552, t6637, t776, t98598, t16758, t22986, t2647, t6646, t5593, t81865, t16924, t23146);
    (t98530, t98534, t98546, t98549, t98553, t98564, t98571, t98575, t98601, t98608, t98610, t98612)
}
