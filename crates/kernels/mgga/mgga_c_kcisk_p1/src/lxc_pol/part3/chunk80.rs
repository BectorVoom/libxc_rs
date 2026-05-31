//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 80/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk80<F: Float>(t12: F, t15: F, t18: F, t26: F, t240: F, t242: F, t34: F, t57: F) -> (F, F, F, F) {
    let t247 = F::cast_from(0.705945e1_f64) * t15 + F::cast_from(0.1549425e1_f64) * t12 + F::cast_from(0.420775e0_f64) * t18 + F::cast_from(0.1562925e0_f64) * t26;
    let t250 = F::cast_from(1.0_f64) + F::cast_from(0.32164683177870697974e2_f64) / t247;
    let t251 = F::ln(t250);
    let t259 = -t34 + t240 * (-F::cast_from(0.3109e-1_f64) * t242 * t251 + t34 - F::cast_from(0.19751789702565206229e-1_f64) * t57) + F::cast_from(0.19751789702565206229e-1_f64) * t240 * t57;
    (t247, t250, t251, t259)
}
