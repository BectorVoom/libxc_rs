//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1752;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta467(t1015: f64, t6746: f64, t23472: f64, t40: f64, t984: f64, t1933: f64, t225: f64, t343: f64, t364: f64, t6721: f64, t6739: f64, t6741: f64, t344: f64, t6729: f64, t6740: f64, t3103: f64, t6755: f64, t3034: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23473, t23474, t23476, t23477, t23478, t23479) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1752(t1015, t6746, t23472, t40, t984, t1933, t225, t343, t364);
        let (t23480, t23482, t23483, t23488, t23489, t23500, t23508) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1753(t23477, t23479, t6721, t6739, t6741, t344, t6729, t6740, t3103, t6755, t3034, t371);
    (t23473, t23474, t23476, t23478, t23479, t23480, t23482, t23483, t23488, t23489, t23500, t23508)
}
