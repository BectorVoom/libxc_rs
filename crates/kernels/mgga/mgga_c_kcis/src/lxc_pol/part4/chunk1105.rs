//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1105/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1105<F: Float>(t22: F, t4864: F, t4715: F, t13710: F, t13712: F, t13723: F, t13732: F, t13767: F, t13939: F, t13942: F, t13945: F, t9726: F, t9729: F) -> (F, F, F) {
    let t13948 = t22 * t4864;
    let t13949 = t13948 * t4715;
    let t13951 = F::new(0.13287407407407407408e0) * t13712 - t13939 + F::new(0.11958666666666666667e1) * t13723 - F::new(0.17938e1) * t13732 - t9726 - t9729 + F::new(0.3071625e0) * t13942 + F::new(0.1898925e1) * t13767 - F::new(0.91285185185185185185e-1) * t13945 - F::new(0.13287407407407407408e0) * t13710 + F::new(0.71202444444444444443e0) * t13949;
    (t13948, t13949, t13951)
}
