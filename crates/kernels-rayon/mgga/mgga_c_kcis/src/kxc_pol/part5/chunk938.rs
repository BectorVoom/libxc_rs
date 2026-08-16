//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 938/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk938(t119: f64, t122: f64, t144: f64, t145: f64, t2552: f64, t2559: f64, t2572: f64, t784: f64, t788: f64, t85: f64, t9070: f64, t9074: f64, t9078: f64, t9082: f64, t9089: f64, t9092: f64, t9094: f64, t9097: f64, t9099: f64, t9103: f64, t9105: f64, t9109: f64) -> f64 {
    let t9112 = -0.1857375e-1_f64 * t784 * t2572 + 0.619125e-2_f64 * t9070 * t145 - 0.8255e-2_f64 * t2552 * t9074 + 0.371475e-1_f64 * t2559 * t9078 - 0.38523333333333333333e-1_f64 * t788 * t9082 - 0.23583209876543209876e-1_f64 * t85 * t119 * t122 - 0.371475e-1_f64 * t144 * t9089 + 0.371475e-1_f64 * t9092 * t9094 + 0.41275e-2_f64 * t9097 * t9099 - 0.74295e-1_f64 * t9103 * t9105 - 0.4953e-1_f64 * t2559 * t9109;
    t9112
}
