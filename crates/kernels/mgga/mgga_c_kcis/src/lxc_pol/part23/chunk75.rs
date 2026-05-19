//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 75/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk75<F: Float>(t12: F, t15: F, t18: F, t26: F, t187: F, t189: F, t34: F, t57: F) -> (F, F, F, F) {
    let t194 = F::new(0.705945e1) * t15 + F::new(0.1549425e1) * t12 + F::new(0.420775e0) * t18 + F::new(0.1562925e0) * t26;
    let t197 = F::new(1.0) + F::cast_from(0.32164683177870697974e2_f64) / t194;
    let t198 = F::ln(t197);
    let t206 = -t34 + t187 * (-F::new(0.3109e-1) * t189 * t198 + t34 - F::cast_from(0.19751789702565206229e-1_f64) * t57) + F::cast_from(0.19751789702565206229e-1_f64) * t187 * t57;
    (t194, t197, t198, t206)
}
