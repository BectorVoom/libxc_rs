//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 670/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk670<F: Float>(t1526: F, t1527: F, t15562: F, t15584: F, t19950: F, t19957: F, t19961: F, t19965: F, t3088: F, t342: F, t343: F, t4415: F, t4422: F, t4501: F, t7704: F) -> F {
    let t19969 = t4415 + t4501 + t7704 - t15562 / F::cast_from(18.0_f64) - t15584 / F::cast_from(6.0_f64) - t1526 * t3088 * t19950 / F::cast_from(9.0_f64) - t1526 * t1527 * t4422 / F::cast_from(6.0_f64) + t1526 * t1527 * t19957 / F::cast_from(6.0_f64) - t1526 * t1527 * t19961 / F::cast_from(12.0_f64) - t342 * t343 * t19965 / F::cast_from(4.0_f64);
    t19969
}
