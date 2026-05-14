//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1418/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1418<F: Float>(t113: F, t115081: F, t125493: F, t125522: F, t125553: F, t125583: F, t125608: F, t125652: F, t125694: F, t125729: F, t125764: F, t125791: F, t1275: F, t128599: F, t128634: F, t128660: F, t128684: F, t128718: F, t128755: F, t1512: F, t16579: F, t18804: F, t18809: F, t18812: F, t25504: F, t29429: F, t31973: F, t332: F, t4377: F, t4385: F, t4391: F, t4395: F, t5: F, t505: F, t5480: F, t5483: F, t6403: F) -> (F,) {
    let t128779 = t6403 * t18804 / 4.0 + t25504 * t5483 / 2.0 + t29429 * t4391 / 2.0 - t29429 * t4395 + t5 * t31973 * t505 / 4.0 + t5 * (t125493 + t125522 + t125553 + t125583 + t125608 + t125652 + t125694 + t125729 + t125764 + t125791 + t128599 + t128634 + t128660 + t128684 + t128718 + t128755) * t332 * t113 / 4.0 + t29429 * t4377 / 2.0 + t29429 * t4385 / 2.0 + t6403 * t18809 / 2.0 + t5 * t1512 * t16579 / 4.0 + t6403 * t18812 / 4.0 + t25504 * t5480 / 4.0 + t115081 * t1275 / 2.0;
    (t128779,)
}
