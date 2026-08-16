//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2635/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2635(t12364: f64, t5234: f64, t1354: f64, t16288: f64, t3858: f64, t12365: f64, t5289: f64, t1827: f64, t39955: f64, t16261: f64, t16398: f64, t12289: f64, t1336: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54532 = t5234 * t12364;
    let t54533 = t54532 * t1354;
    let t54535 = t16288 * t3858;
    let t54555 = t12365 * t5289;
    let t54557 = t39955 * t1827;
    let t54561 = t16398 * t16261;
    let t54566 = t1336 * t12289 * t836;
    (t54532, t54533, t54535, t54555, t54557, t54561, t54566)
}
