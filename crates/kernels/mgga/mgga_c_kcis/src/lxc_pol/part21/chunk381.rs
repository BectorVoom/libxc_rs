//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 381/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk381<F: Float>(t2209: F, t236: F, t119: F, t32: F, t5: F, t645: F, t88: F, t237: F, t663: F, t28: F, t644: F, t14: F) -> (F, F, F, F, F, F) {
    let t2210 = t236 * t2209;
    let t2302 = F::cast_from(0.14764770444444444444e-2_f64) * t5 * t119 * t32;
    let t2303 = t88 * t645;
    let t2306 = F::cast_from(0.35616666666666666667e-1_f64) * t237 * t2303 * t663;
    let t2307 = t644 * t28;
    let t2308 = F::new(1.0) / t2307;
    let t2309 = t14 * t2308;
    (t2210, t2302, t2303, t2306, t2308, t2309)
}
