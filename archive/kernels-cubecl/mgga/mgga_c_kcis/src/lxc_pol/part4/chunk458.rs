//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 458/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk458<F: Float>(t1212: F, t1225: F, t1233: F, t1668: F, t1682: F, t1685: F, t1694: F, t1823: F, t1831: F, t1835: F, t187: F, t405: F) -> F {
    let t1844 = -t1668 + t1682 + t187 * (-F::cast_from(0.3109e-1_f64) * t1823 * t405 + F::cast_from(1.0_f64) * t1212 * t1831 + t1668 - t1682 - F::cast_from(0.19751789702565206229e-1_f64) * t1685 + F::cast_from(0.58482233974552040708e0_f64) * t1225 * t1835) + F::cast_from(0.19751789702565206229e-1_f64) * t187 * t1685 - F::cast_from(0.58482233974552040708e0_f64) * t1233 * t1694;
    t1844
}
