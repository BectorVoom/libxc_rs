//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 20/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk20<F: Float>(t12: F, t15: F, t18: F, t26: F, t14: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t28 = F::new(0.379785e1) * t15 + F::new(0.8969e0) * t12 + F::new(0.204775e0) * t18 + F::new(0.123235e0) * t26;
    let t31 = F::new(1.0) + F::cast_from(0.16081824322151104822e2_f64) / t28;
    let t32 = F::ln(t31);
    let t34 = F::new(0.62182e-1) * t14 * t32;
    let t36 = pow_1_3::<F>(zeta_threshold);
    let t37 = t36 * zeta_threshold;
    (t28, t31, t32, t34, t37)
}
