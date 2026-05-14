//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1330/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1330<F: Float>(t31804: F, t8392: F, t113286: F, t3886: F, t113279: F, t19338: F, t6334: F, t18712: F, t25140: F, t25037: F, t10703: F, t112663: F, t112760: F, t112888: F, t114847: F, t15229: F, t15290: F, t15299: F, t1901: F, t19399: F, t19435: F, t19486: F, t19526: F, t19546: F, t19572: F, t19617: F, t19622: F, t19626: F, t24886: F, t29093: F, t29128: F, t29129: F, t6273: F) -> (F, F, F, F, F, F) {
    let t126428 = t8392 * t31804;
    let t126447 = t113286 * t3886;
    let t126451 = t113279 * t3886;
    let t126455 = t6334 * t19338;
    let t126463 = t25140 * t18712;
    let t126467 = t25037 * t18712;
    let t126471 = t1901 * t24886 * t19572 / 9.0 + 2.0 / 27.0 * t1901 * t29093 * t19546 + 4.0 / 9.0 * t1901 * t29093 * t19486 - 2.0 / 27.0 * t126428 - 4.0 * t1901 * t29128 * t29129 * t19435 + 8.0 * t1901 * t112888 * t6273 * t19399 - 4.0 / 9.0 * t1901 * t112760 * t19617 - 4.0 / 9.0 * t1901 * t114847 * t19622 + 4.0 / 27.0 * t1901 * t112663 * t19626 - 4.0 / 9.0 * t1901 * t15229 * t126447 + 4.0 / 27.0 * t1901 * t15290 * t126451 - 2.0 / 9.0 * t1901 * t15299 * t126455 - t1901 * t10703 * t6334 * t19526 / 9.0 - 2.0 / 9.0 * t1901 * t15229 * t126463 + 2.0 / 27.0 * t1901 * t15290 * t126467;
    (t126447, t126451, t126455, t126463, t126467, t126471)
}
