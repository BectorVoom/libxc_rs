//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1332/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1332<F: Float>(t1882: F, t30311: F, t1060: F, t107477: F, t107478: F, t107499: F, t107519: F, t107533: F, t1384: F, t15772: F, t16919: F, t1901: F, t2185: F, t2221: F, t23478: F, t26909: F, t26950: F, t30232: F, t30431: F, t41107: F, t446: F, t4733: F, t574: F, t5855: F, t605: F, t616: F, t9432: F, t96215: F, t96220: F) -> (F,) {
    let t121533 = t1882 * t30311;
    let t121541 = -2.0 * t446 * t9432 * t616 * t30232 + t1901 * t2221 * t5855 * t15772 / 9.0 + 2.0 / 27.0 * t1901 * t41107 * t30431 - t107477 + 4.0 / 27.0 * t107478 - t107499 + 2.0 / 3.0 * t446 * t574 * t23478 * t4733 + 4.0 / 3.0 * t446 * t2185 * t1060 * t26909 + t446 * t574 * t605 * t1384 * t16919 / 3.0 + t121533 / 9.0 - t96215 + 8.0 / 27.0 * t107519 - t107533 - 4.0 / 27.0 * t96220 + 4.0 / 3.0 * t446 * t2185 * t1060 * t26950;
    (t121541,)
}
