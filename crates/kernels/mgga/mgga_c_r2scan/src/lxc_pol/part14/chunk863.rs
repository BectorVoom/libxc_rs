//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 863/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk863<F: Float>(t5986: F, t5834: F, t5963: F, t5966: F, t5968: F, t5970: F, t5972: F, t5975: F, t5976: F, t5978: F, t5982: F, t5985: F) -> F {
    let t7849 = F::new(80.0) * t5986;
    let t7850 = t5963 - t5966 + F::cast_from(0.43374325201206959368e-1_f64) * t5968 - F::cast_from(0.64212977516902094772e0_f64) * t5970 - F::cast_from(0.2602459512072417562e0_f64) * t5972 - t5975 + F::new(16.0) * t5976 - F::cast_from(0.2258170631111111111e-2_f64) * t5978 + t5834 - F::new(40.0) * t5982 + t5985 - t7849;
    t7850
}
