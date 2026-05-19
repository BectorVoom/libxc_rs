//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 819/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk819<F: Float>(t2850: F, t2880: F, t298: F, t142: F, t2884: F, t2888: F, t2917: F, t840: F, t2920: F, t55: F, t12535: F, t2879: F, t2885: F, t2887: F, t829: F) -> (F, F, F, F, F, F) {
    let t12604 = F::new(0.53425e-1) * t298 * t2850 * t2880;
    let t12605 = t142 * t2884;
    let t12608 = F::cast_from(0.85917146441092277512e0_f64) * t298 * t12605 * t2888;
    let t12610 = F::new(1.0) / t2917 / t840;
    let t12613 = F::new(1.0) / t2920 / t55;
    let t12614 = t12610 * t12535 * t12613;
    let t12620 = F::cast_from(0.48245472966453314466e2_f64) * t2885 * t2879 * t2887 * t829;
    (t12604, t12608, t12610, t12613, t12614, t12620)
}
