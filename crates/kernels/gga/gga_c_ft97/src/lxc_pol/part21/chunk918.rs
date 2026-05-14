//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 918/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk918<F: Float>(t27351: F, t27364: F, t27376: F, t27389: F, t143: F, t160: F, t376: F, t6687: F, t89: F, t144: F, t26529: F, t1901: F, t24003: F, t24004: F, t24007: F, t24054: F, t27313: F, t27316: F, t27320: F, t27324: F, t27326: F, t27330: F, t27337: F, t28: F, t446: F) -> (F, F, F, F, F) {
    let t27391 = t27351 + t27364 + t27376 + t27389;
    let t27393 = t143 * t27391 * t160;
    let t27398 = t89 * t376 * t6687;
    let t27400 = t144 * t26529;
    let t27403 = -t24003 - t446 * t27313 / 3.0 - t446 * t27316 / 3.0 + t24004 / 9.0 - t446 * t27320 / 3.0 - t24007 / 9.0 + t27324 / 9.0 + t1901 * t27326 / 9.0 - 2.0 / 3.0 * t1901 * t27330 - 2.0 * t1901 * t27337 + t89 * t28 * t27393 / 3.0 - t27398 / 9.0 + t24054 + 2.0 / 3.0 * t446 * t27400;
    (t27391, t27393, t27398, t27400, t27403)
}
