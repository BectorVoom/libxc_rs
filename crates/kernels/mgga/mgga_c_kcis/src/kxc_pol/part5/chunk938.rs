//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 938/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk938<F: Float>(t119: F, t122: F, t144: F, t145: F, t2552: F, t2559: F, t2572: F, t784: F, t788: F, t85: F, t9070: F, t9074: F, t9078: F, t9082: F, t9089: F, t9092: F, t9094: F, t9097: F, t9099: F, t9103: F, t9105: F, t9109: F) -> F {
    let t9112 = -F::cast_from(0.1857375e-1_f64) * t784 * t2572 + F::cast_from(0.619125e-2_f64) * t9070 * t145 - F::cast_from(0.8255e-2_f64) * t2552 * t9074 + F::cast_from(0.371475e-1_f64) * t2559 * t9078 - F::cast_from(0.38523333333333333333e-1_f64) * t788 * t9082 - F::cast_from(0.23583209876543209876e-1_f64) * t85 * t119 * t122 - F::cast_from(0.371475e-1_f64) * t144 * t9089 + F::cast_from(0.371475e-1_f64) * t9092 * t9094 + F::cast_from(0.41275e-2_f64) * t9097 * t9099 - F::cast_from(0.74295e-1_f64) * t9103 * t9105 - F::cast_from(0.4953e-1_f64) * t2559 * t9109;
    t9112
}
