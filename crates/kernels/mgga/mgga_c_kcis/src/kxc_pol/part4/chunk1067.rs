//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1067/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1067<F: Float>(t1056: F, t13480: F, t10021: F, t10026: F, t10028: F, t10033: F, t10035: F, t10037: F, t10045: F, t10048: F, t10056: F, t10058: F, t111: F, t120: F, t13463: F, t13468: F, t13472: F, t13473: F, t13476: F, t3158: F, t4875: F) -> F {
    let t13481 = t1056 * t13480;
    let t13484 = -F::new(0.10359077815592613752e-3) * t4875 + F::new(0.26416666666666666666e-2) * t10021 + F::new(0.23526125e-4) * t10026 - F::new(0.9368e-2) * t10028 + F::new(0.78420416666666666666e-4) * t10033 + F::new(0.4684e-2) * t10035 - F::new(0.15613333333333333333e-2) * t10037 - F::new(0.13208333333333333333e-2) * t10045 + F::new(0.88055555555555555553e-3) * t10048 - F::new(0.117630625e-4) * t10056 + F::new(0.15684083333333333333e-4) * t10058 + F::new(0.23911438650126355246e-1) * t3158 - F::new(0.52833333333333333333e-3) * t111 * t13463 - F::new(0.17611111111111111111e-3) * t111 * t13468 + t13472 + F::new(0.31368166666666666666e-4) * t13473 - F::new(0.10082625e-4) * t120 * t13476 + F::new(0.403305e-4) * t120 * t13481;
    t13484
}
