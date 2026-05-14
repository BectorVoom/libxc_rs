//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1391/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1391<F: Float>(t26389: F, t21102: F, t21104: F, t21107: F, t21110: F, t21112: F, t21119: F, t26374: F, t26376: F, t26378: F, t26382: F, t26384: F, t26386: F, t26388: F, t21224: F, t5249: F, t898: F) -> (F, F) {
    let t26390 = 240.0 * t26389;
    let t26392 = 0.1524265176e-1 * t26374 + 0.67745118933333333331e-2 * t26376 - 0.16867793133802706421e-1 * t26378 - t26382 - 0.254044196e-2 * t26384 - 0.50808839199999999999e-2 * t26386 + t26388 - t21102 - t26390 - t21104 + t21107 - t21110 - 3.0 * t21112 + t21119;
    let t26396 = t5249 * t898 * t21224;
    (t26392, t26396)
}
