//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 899/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk899<F: Float>(t3728: F, t5749: F, t5753: F, t11913: F, t5645: F, t16694: F, t5662: F, t4170: F, t4160: F, t1984: F, t3245: F, t12234: F, t1943: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16804 = t3728 * t5749;
    let t16805 = F::new(0.33163888888888888888e-2) * t16804;
    let t16806 = t3728 * t5753;
    let t16808 = t11913 * t5645;
    let t16809 = F::new(0.22109259259259259258e-2) * t16808;
    let t16810 = t5662 * t16694;
    let t16811 = t4170 * t16810;
    let t16812 = t4160 * t16811;
    let t16820 = t3245 * t1984;
    let t16823 = t1943 * t12234;
    (t16804, t16805, t16806, t16808, t16809, t16810, t16812, t16820, t16823)
}
