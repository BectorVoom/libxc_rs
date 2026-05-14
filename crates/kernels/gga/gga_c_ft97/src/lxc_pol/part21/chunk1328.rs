//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1328/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1328<F: Float>(t30397: F, t8392: F, t30405: F, t30428: F, t30432: F, t106551: F, t107180: F, t107183: F, t107193: F, t107210: F, t107234: F, t107236: F, t107241: F, t12968: F, t1901: F, t30412: F, t30416: F, t3430: F, t4733: F, t50268: F, t9099: F, t95813: F) -> (F,) {
    let t121396 = t8392 * t30397;
    let t121398 = t8392 * t30405;
    let t121400 = t8392 * t30428;
    let t121402 = t8392 * t30432;
    let t121404 = -2.0 / 9.0 * t1901 * t9099 * t30412 - t107180 + t107183 + t107193 + t107210 + 2.0 / 9.0 * t1901 * t106551 * t3430 - 4.0 / 3.0 * t1901 * t50268 * t30416 - 4.0 / 3.0 * t1901 * t12968 * t95813 * t4733 + t107234 - 2.0 / 27.0 * t121396 - t121398 / 27.0 - t121400 / 27.0 + t107236 - 2.0 / 81.0 * t121402 - t107241;
    (t121404,)
}
