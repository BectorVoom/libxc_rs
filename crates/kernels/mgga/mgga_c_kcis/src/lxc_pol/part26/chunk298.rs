//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 298/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk298<F: Float>(t1313: F, t1338: F, t1342: F, t1357: F, t1559: F, t1564: F, t1573: F, t1577: F, t1578: F, t1585: F, t187: F, t601: F) -> F {
    let t1588 = -t1313 + t1338 + t187 * (-F::cast_from(0.3109e-1_f64) * t1559 * t601 + F::cast_from(1.0_f64) * t1564 * t1573 + t1313 - t1338 - F::cast_from(0.19751789702565206229e-1_f64) * t1342 + F::cast_from(0.58482233974552040708e0_f64) * t1577 * t1578) + F::cast_from(0.19751789702565206229e-1_f64) * t187 * t1342 - F::cast_from(0.58482233974552040708e0_f64) * t1585 * t1357;
    t1588
}
