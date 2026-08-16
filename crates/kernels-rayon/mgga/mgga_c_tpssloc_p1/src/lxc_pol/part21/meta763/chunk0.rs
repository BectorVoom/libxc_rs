//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2638/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2638(t2585: f64, t3732: f64, t46853: f64, t5308: f64, t16118: f64, t9577: f64, t212: f64, t5187: f64, t12225: f64, t2586: f64, t16100: f64, t782: f64) -> (f64, f64, f64, f64, f64) {
    let t54643 = t2585 * t3732 * t46853 * t5308;
    let t54663 = t9577 * t16118;
    let t54665 = t212 * t5187;
    let t54667 = t2586 * t12225 * t54665;
    let t54670 = t782 * t16100;
    (t54643, t54663, t54665, t54667, t54670)
}
