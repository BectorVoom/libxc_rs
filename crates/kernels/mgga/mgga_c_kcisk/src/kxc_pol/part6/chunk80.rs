//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 80/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk80<F: Float>(t12: F, t15: F, t18: F, t26: F, t240: F, t242: F, t34: F, t57: F) -> (F, F, F, F) {
    let t247 = F::new(0.705945e1) * t15 + F::new(0.1549425e1) * t12 + F::new(0.420775e0) * t18 + F::new(0.1562925e0) * t26;
    let t250 = F::new(1.0) + F::cast_from(0.32164683177870697974e2_f64) / t247;
    let t251 = F::ln(t250);
    let t259 = -t34 + t240 * (-F::new(0.3109e-1) * t242 * t251 + t34 - F::cast_from(0.19751789702565206229e-1_f64) * t57) + F::cast_from(0.19751789702565206229e-1_f64) * t240 * t57;
    (t247, t250, t251, t259)
}
