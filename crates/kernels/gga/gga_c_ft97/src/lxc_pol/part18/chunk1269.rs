//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1269/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1269<F: Float>(t86: F, t100086: F, t100120: F, t100163: F, t101938: F, t101982: F, t102021: F, t102054: F, t102283: F, t102328: F, t102370: F, t102399: F, t102428: F, t103957: F, t103981: F, t104017: F, t104049: F, t113: F, t1342: F, t1577: F, t18: F, t1934: F, t23388: F, t26498: F, t26508: F, t5: F, t505: F, t5756: F, t6570: F, t7742: F, t992: F) -> (F,) {
    let t87 = 10000000.0 <= t86;
    let t104076 = piecewise3(t87, 0.0, t5 * (t100086 + t100120 + t100163 + t101938 + t101982 + t102021 + t102054 + t102283 + t102328 + t102370 + t102399 + t102428 + t103957 + t103981 + t104017 + t104049) * t113 / 4.0 + t5 * t26498 * t505 / 2.0 + t5 * t6570 * t1934 / 4.0 + t5 * t23388 * t992 / 4.0 - t5 * t5756 * t18 * t1577 - t5 * t1342 * t1577 / 2.0 + 3.0 / 2.0 * t5 * t26508 * t7742);
    (t104076,)
}
