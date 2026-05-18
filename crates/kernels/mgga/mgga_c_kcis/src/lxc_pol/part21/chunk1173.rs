//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1173/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1173<F: Float>(t236: F, t28300: F, t233: F, t27741: F, t10819: F, t1259: F, t3530: F, t3622: F, t779: F, t9274: F, t2531: F, t2537: F) -> (F, F, F, F, F, F, F) {
    let t28301 = t236 * t28300;
    let t28302 = t233 * t28301;
    let t28901 = F::new(2.0) * t27741;
    let t30045 = t1259 * t10819;
    let t30066 = t3530 * t3622;
    let t31271 = t779 * t9274;
    let t31274 = t2531 * t2537;
    (t28301, t28302, t28901, t30045, t30066, t31271, t31274)
}
