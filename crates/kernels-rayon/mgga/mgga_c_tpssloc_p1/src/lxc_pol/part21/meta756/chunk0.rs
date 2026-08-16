//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2630/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2630(t12283: f64, t16308: f64, t1824: f64, t3791: f64, t12300: f64, t5289: f64, t16208: f64, t3799: f64, t1788: f64, t9212: f64, t9214: f64, t2223: f64, t5168: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54237 = t12283 * t16308;
    let t54258 = t1824 * t3791;
    let t54293 = t12300 * t5289;
    let t54295 = t3799 * t16208;
    let t54312 = t9212 * t1788;
    let t54314 = t9214 * t1788;
    let t54316 = t2223 * t5168;
    (t54237, t54258, t54293, t54295, t54312, t54314, t54316)
}
