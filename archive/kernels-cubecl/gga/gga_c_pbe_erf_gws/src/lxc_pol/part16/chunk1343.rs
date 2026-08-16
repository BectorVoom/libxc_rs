//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1343/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1343<F: Float>(t54344: F, t54352: F, t54354: F, t54356: F, t51431: F, t54338: F, t54342: F, t54346: F, t54348: F, t54350: F, t54360: F, t54362: F) -> F {
    let t55603 = F::cast_from(35.0_f64) / F::cast_from(144.0_f64) * t54344;
    let t55607 = F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t54352;
    let t55608 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t54354;
    let t55609 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t54356;
    let t55613 = -F::cast_from(5.0_f64) / F::cast_from(48.0_f64) * t54338 + t54342 / F::cast_from(24.0_f64) - t55603 - F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t54346 - t54348 / F::cast_from(24.0_f64) - t54350 / F::cast_from(48.0_f64) - t55607 + t55608 - t55609 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t51431 + t54360 / F::cast_from(4.0_f64) + t54362 / F::cast_from(192.0_f64);
    t55613
}
