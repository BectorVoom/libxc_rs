//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 961/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk961<F: Float>(t1073: F, t12112: F, t17567: F, t21058: F, t2258: F, t2259: F, t2265: F, t2271: F, t48442: F, t4883: F, t631: F, t632: F, t637: F, t65113: F, t72: F, t76221: F, t76232: F, t76238: F, t76241: F, t76265: F, t76302: F, t85451: F, t85469: F, t85501: F, t8660: F) -> (F,) {
    let t87941 = 12.0 * t76221 + 12.0 * t2265 * t12112 * t21058 - 16.0 / 81.0 * t76232 + 8.0 / 9.0 * t76238 - 10.0 * t65113 - 16.0 * t76241 - 4.0 * t631 * t72 * t8660 * t85469 - t631 * t72 * t2271 * t85451 + t631 * t2258 * t2259 * t85451 / 6.0 + t631 * t72 * t632 * t85501 / 6.0 + 36.0 * t631 * t637 * t17567 * t4883 - 160.0 / 27.0 * t48442 - 4.0 / 3.0 * t76265 - 6.0 * t631 * t637 * t76302 * t1073;
    (t87941,)
}
