//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1138/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1138(t1591: f64, t17960: f64, t4413: f64, t6136: f64, t12857: f64, t2093: f64, t51622: f64, t5737: f64, t286: f64, t69: f64, t1610: f64, t167: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54581 = t17960 * t1591;
    let t54605 = t6136 * t4413;
    let t54624 = t2093 * t12857;
    let t59319 = t51622 * t5737;
    let t61287 = t69 * t286;
    let t61402 = t1610 * t167;
    (t54581, t54605, t54624, t59319, t61287, t61402)
}
