//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 986/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk986<F: Float>(t26179: F, t7706: F, t7349: F, t7709: F, t13272: F, t7342: F, t2048: F, t26180: F, t26185: F, t26187: F, t28105: F, t28109: F, t28112: F, t28116: F, t28119: F, t28141: F, t6960: F, t7343: F, t7352: F) -> F {
    let t28598 = t26179 * t7706;
    let t28600 = t7709 * t7349;
    let t28602 = t13272 * t7342;
    let t28621 = F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t26180 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t26185 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t28598 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t28600 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t28602 * t6960 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28141 * t2048 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t26187 * t7706 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7343 * t28105 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7343 * t28109 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28112 * t2048 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28116 * t2048 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28119 * t2048 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7709 * t7352;
    t28621
}
