//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1335/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1335<F: Float>(t1017: F, t107625: F, t107650: F, t107670: F, t107680: F, t107683: F, t107685: F, t107691: F, t119616: F, t12680: F, t12968: F, t13212: F, t13220: F, t1391: F, t17066: F, t17071: F, t1901: F, t2142: F, t23455: F, t27021: F, t27414: F, t30133: F, t30383: F, t379: F, t446: F, t4714: F, t574: F, t5968: F, t605: F, t9432: F) -> (F,) {
    let t121639 = -8.0 / 27.0 * t107625 + 2.0 / 27.0 * t1901 * t13212 * t119616 - 2.0 * t446 * t9432 * t1391 * t17066 + 16.0 / 27.0 * t107650 - 2.0 / 9.0 * t1901 * t13220 * t30133 * t379 + 2.0 / 9.0 * t1901 * t12680 * t27021 - t107670 - 2.0 / 3.0 * t446 * t574 * t27414 * t1017 + t107680 + 4.0 / 3.0 * t1901 * t12968 * t23455 * t17071 - t107683 + 8.0 / 27.0 * t107685 + t446 * t574 * t2142 * t30383 / 3.0 + t446 * t574 * t605 * t5968 * t4714 / 3.0 + t107691;
    (t121639,)
}
