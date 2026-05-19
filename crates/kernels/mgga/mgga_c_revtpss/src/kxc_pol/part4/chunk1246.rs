//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1246/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1246<F: Float>(t11326: F, t15108: F, t15111: F, t15114: F, t15116: F, t15119: F, t15121: F, t15123: F, t15127: F, t15132: F, t15178: F, t15181: F, t15184: F, t15187: F, t15189: F, t15195: F, t15200: F, t15435: F, t15450: F, t15457: F, t15459: F, t15472: F) -> F {
    let t15474 = F::cast_from(0.142419375e1_f64) * t15108 - F::new(0.76790625e-1) * t15111 - F::new(0.1898925e1) * t15114 - F::new(0.9494625e0) * t15116 + F::new(0.3071625e0) * t15119 + F::new(0.15358125e0) * t15121 - F::cast_from(0.91285185185185185185e-1_f64) * t15123 - t15435 + F::cast_from(0.13287407407407407408e0_f64) * t15127 - F::cast_from(0.39862222222222222222e0_f64) * t15132 + t15450 - F::cast_from(0.27385555555555555556e-1_f64) * t15178 - F::cast_from(0.36514074074074074075e-1_f64) * t15181 + F::cast_from(0.32862666666666666666e0_f64) * t15184 + F::cast_from(0.16431333333333333333e0_f64) * t15187 - F::cast_from(0.13287407407407407408e0_f64) * t15189 + t15457 - F::cast_from(0.29896666666666666667e0_f64) * t15195 + t15459 - F::cast_from(0.82156666666666666667e-1_f64) * t15200 - F::cast_from(0.10954222222222222222e0_f64) * t11326 + t15472;
    t15474
}
