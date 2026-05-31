//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 20/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk20<F: Float>(t12: F, t15: F, t18: F, t26: F, t14: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t28 = F::cast_from(0.379785e1_f64) * t15 + F::cast_from(0.8969e0_f64) * t12 + F::cast_from(0.204775e0_f64) * t18 + F::cast_from(0.123235e0_f64) * t26;
    let t31 = F::cast_from(1.0_f64) + F::cast_from(0.16081824322151104822e2_f64) / t28;
    let t32 = F::ln(t31);
    let t34 = F::cast_from(0.62182e-1_f64) * t14 * t32;
    let t36 = pow_1_3::<F>(zeta_threshold);
    let t37 = t36 * zeta_threshold;
    (t28, t31, t32, t34, t37)
}
