//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 806/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk806<F: Float>(t1035: F, t6352: F, t2918: F, t4612: F, t6328: F, t6332: F, t6336: F, t261: F, t1680: F, t4685: F, t1679: F, t950: F) -> (F, F, F, F, F, F) {
    let t6353 = t1035 * t6352;
    let t6360 = t2918 + F::cast_from(0.11872222222222222222e-1_f64) * t4612 - F::cast_from(0.11872222222222222222e-1_f64) * t6328 + F::cast_from(0.35616666666666666666e-1_f64) * t6332 - F::cast_from(0.17808333333333333333e-1_f64) * t6336;
    let t6362 = F::new(0.62182e-1) * t6360 * t261;
    let t6364 = F::new(2.0) * t4685 * t1680;
    let t6365 = t1679 * t1679;
    let t6366 = t6365 * t950;
    (t6353, t6360, t6362, t6364, t6365, t6366)
}
