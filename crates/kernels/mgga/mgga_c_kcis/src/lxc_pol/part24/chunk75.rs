//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 75/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk75<F: Float>(t12: F, t15: F, t18: F, t26: F, t187: F, t189: F, t34: F, t57: F) -> (F, F, F, F) {
    let t194 = 0.705945e1 * t15 + 0.1549425e1 * t12 + 0.420775e0 * t18 + 0.1562925e0 * t26;
    let t197 = 1.0 + 0.32164683177870697974e2 / t194;
    let t198 = f64::ln(t197);
    let t206 = -t34 + t187 * (-0.3109e-1 * t189 * t198 + t34 - 0.19751789702565206229e-1 * t57) + 0.19751789702565206229e-1 * t187 * t57;
    (t194, t197, t198, t206)
}
