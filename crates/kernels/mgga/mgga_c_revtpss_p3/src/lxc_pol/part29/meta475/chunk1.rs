//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1749/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1749<F: Float>(t1459: F, t1461: F, t2113: F, t2115: F, t26716: F, t26730: F, t26734: F, t26737: F, t26740: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, t7547: F, t7554: F, t7557: F) -> F {
    let t26743 = F::cast_from(12.0_f64) * t1459 * t7554 + F::cast_from(6.0_f64) * t1459 * t7557 + F::cast_from(6.0_f64) * t1461 * t7547 + F::cast_from(6.0_f64) * t2113 * t4162 + F::cast_from(3.0_f64) * t2113 * t4165 + F::cast_from(3.0_f64) * t2115 * t4158 + t26716 * t573 + F::cast_from(6.0_f64) * t26730 * t572 + F::cast_from(12.0_f64) * t26734 * t572 + F::cast_from(6.0_f64) * t26737 * t572 + F::cast_from(3.0_f64) * t26740 * t572;
    t26743
}
