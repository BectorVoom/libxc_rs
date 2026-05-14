//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1299/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1299<F: Float>(t1882: F, t30369: F, t16977: F, t23997: F, t30409: F, t8392: F, t1045: F, t106314: F, t106319: F, t11593: F, t119443: F, t119484: F, t119486: F, t12982: F, t144: F, t1901: F, t2221: F, t23581: F, t27000: F, t27006: F, t30408: F, t30427: F, t3052: F, t446: F, t4462: F, t9016: F, t9419: F, t95446: F, t95448: F) -> (F, F) {
    let t120327 = t1882 * t30369;
    let t120335 = t23997 * t16977;
    let t120362 = t8392 * t30409;
    let t120364 = 2.0 / 9.0 * t120327 - 2.0 * t446 * t144 * t119484 + 4.0 / 3.0 * t446 * t144 * t119486 + 2.0 / 3.0 * t446 * t144 * t120335 + t1901 * t9419 * t30427 / 9.0 + t1901 * t2221 * t23581 * t4462 / 9.0 + 4.0 / 9.0 * t11593 * t2221 * t27006 * t3052 + 2.0 / 27.0 * t1901 * t12982 * t30408 - 2.0 / 3.0 * t446 * t144 * t119443 + 8.0 / 27.0 * t95446 + 4.0 / 27.0 * t95448 - 4.0 * t1901 * t9016 * t1045 * t27000 - 2.0 / 81.0 * t120362 - t106314 - t106319;
    (t120335, t120364)
}
