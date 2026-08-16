//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1037/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1037(t1494: f64, t3570: f64, t1498: f64, t3431: f64, t4708: f64, t13087: f64, t4904: f64, t14220: f64, t4425: f64, t4741: f64, t1163: f64, t13889: f64, t1540: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17851 = t3570 * t1494;
    let t17853 = t3570 * t1498;
    let t17855 = t3431 * t4708;
    let t17857 = t13087 * t4904;
    let t17859 = t14220 * t4425;
    let t17861 = t14220 * t4741;
    let t17868 = t1163 * t13889 * t1540;
    (t17851, t17853, t17855, t17857, t17859, t17861, t17868)
}
