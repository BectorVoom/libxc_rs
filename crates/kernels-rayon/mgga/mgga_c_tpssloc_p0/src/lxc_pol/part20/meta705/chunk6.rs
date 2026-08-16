//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2684/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2684(t16118: f64, t9577: f64, t212: f64, t5187: f64, t12225: f64, t2586: f64, t16100: f64, t782: f64, t16103: f64, t16081: f64, t16090: f64, t16093: f64, t16097: f64, t2566: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54663 = t9577 * t16118;
    let t54665 = t212 * t5187;
    let t54667 = t2586 * t12225 * t54665;
    let t54668 = 0.49999999999999999998e-2_f64 * t54667;
    let t54670 = t782 * t16100;
    let t54671 = t54670 * t16103;
    let t54673 = t16081 * t16090;
    let t54676 = t2566 * t16093 * t16097;
    (t54663, t54665, t54668, t54671, t54673, t54676)
}
