//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 865/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk865<F: Float>(t9176: F, t11551: F, t3257: F, t3803: F, t12039: F, t13124: F, t4394: F, t6608: F, t6610: F, t860: F, t9182: F, t13172: F, t824: F) -> (F, F, F, F, F, F, F, F) {
    let t13569 = F::new(35.0) / F::new(72.0) * t9176;
    let t13571 = t3257 * t3803 * t11551;
    let t13575 = F::new(7.0) / F::new(96.0) * t12039;
    let t13578 = t13124 * t4394;
    let t13580 = t6608 * t13578 * t6610;
    let t13582 = t13580 * t860 / F::new(96.0);
    let t13583 = F::new(35.0) / F::new(144.0) * t9182;
    let t13585 = t13172 * t824;
    (t13569, t13571, t13575, t13578, t13580, t13582, t13583, t13585)
}
