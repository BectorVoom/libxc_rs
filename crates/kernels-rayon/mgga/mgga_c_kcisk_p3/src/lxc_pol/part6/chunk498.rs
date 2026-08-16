//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 498/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk498(t3845: f64, t698: f64, t445: f64, t5082: f64, t213: f64, t695: f64, t1849: f64, t967: f64, t167: f64, t4597: f64, t1797: f64, t704: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5126 = t3845 * t698;
    let t5128 = 0.16804375e-4_f64 * t445 * t5126;
    let t5129 = 0.23911438650126355246e-1_f64 * t5082;
    let t5134 = t213 * t695;
    let t5135 = 0.15538616723388920628e-3_f64 * t5134;
    let t5136 = t967 * t1849;
    let t5168 = t167 * t4597;
    let t5180 = t1797 * t704;
    (t5128, t5129, t5135, t5136, t5168, t5180)
}
