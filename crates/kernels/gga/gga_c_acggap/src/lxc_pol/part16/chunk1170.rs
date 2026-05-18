//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1170/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1170<F: Float>(t1805: F, t7329: F, t2001: F, t5539: F, t31346: F, t6328: F, t6140: F, t31525: F, t31526: F, t31530: F, t31532: F, t31543: F, t31544: F, t35723: F, t35737: F, t35748: F, t35756: F, t37694: F, t37697: F, t37698: F, t37700: F) -> F {
    let t40126 = t7329 * t1805;
    let t40131 = t2001 * t5539;
    let t40134 = t31346 * t6328;
    let t40136 = t31346 * t6140;
    let t40138 = F::new(7.0) / F::new(144.0) * t40126 + t31525 + F::new(0.19812298142450615803e-1) * t31526 + F::new(0.17149607247227894789e-2) * t31530 - F::new(0.17149607247227894789e-2) * t31532 + t31543 + t35723 - t37694 + F::new(0.51448821741683684366e-2) * t40131 + F::new(0.33020496904084359672e-1) * t31544 - t35737 + t37697 + F::new(0.13719685797782315831e-1) * t40134 - F::new(0.20579528696673473747e-1) * t40136 + t37698 - t37700 - t35748 + t35756;
    t40138
}
