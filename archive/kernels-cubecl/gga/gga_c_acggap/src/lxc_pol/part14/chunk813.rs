//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 813/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk813<F: Float>(t8841: F, t8847: F, t8851: F, t8860: F, t8876: F, t8879: F, t8882: F, t8898: F, t8945: F, t8973: F, t8981: F, t1713: F, t469: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9313 = F::cast_from(0.17149607247227894789e-2_f64) * t8841;
    let t9316 = F::cast_from(0.17149607247227894789e-2_f64) * t8847;
    let t9318 = F::cast_from(0.21437009059034868486e-3_f64) * t8851;
    let t9320 = F::cast_from(0.14291339372689912324e-3_f64) * t8860;
    let t9328 = t8876 / F::cast_from(32.0_f64);
    let t9329 = t8879 / F::cast_from(96.0_f64);
    let t9331 = F::cast_from(0.5603125e-1_f64) * t8882;
    let t9335 = F::cast_from(0.21437009059034868486e-3_f64) * t8898;
    let t9348 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t8945;
    let t9356 = F::cast_from(0.64311027177104605458e-2_f64) * t8973;
    let t9359 = F::cast_from(0.94344276868812456204e-2_f64) * t8981;
    let t9469 = t469 * t1713;
    (t9313, t9316, t9318, t9320, t9328, t9329, t9331, t9335, t9348, t9356, t9359, t9469)
}
