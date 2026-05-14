//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1330/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1330<F: Float>(t30543: F, t8392: F, t1882: F, t30483: F, t30555: F, t106573: F, t107361: F, t107370: F, t107379: F, t107381: F, t107412: F, t107417: F, t107418: F, t107448: F, t12968: F, t13140: F, t17086: F, t1901: F, t2179: F, t26999: F, t27006: F, t30477: F, t3450: F, t3455: F, t446: F, t4724: F, t574: F, t5842: F, t9276: F) -> (F,) {
    let t121442 = t8392 * t30543;
    let t121453 = t1882 * t30483;
    let t121455 = t1882 * t30555;
    let t121462 = -4.0 * t1901 * t26999 * t27006 * t3450 - 4.0 / 3.0 * t1901 * t12968 * t106573 * t3455 + t107361 + t107370 - 2.0 / 27.0 * t121442 + t107379 + t107381 - 2.0 / 3.0 * t446 * t574 * t9276 * t30477 - 2.0 / 3.0 * t446 * t574 * t2179 * t5842 * t4724 + 2.0 / 3.0 * t121453 + t107412 - 2.0 / 9.0 * t121455 + t107417 - 8.0 / 27.0 * t107418 + 2.0 * t1901 * t13140 * t107448 * t17086;
    (t121462,)
}
