//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 857/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk857<F: Float>(t15385: F, t15387: F, t2413: F, t4145: F, t2874: F, t2405: F, t10479: F, t4142: F, t8392: F, t15346: F, t15350: F, t15355: F, t15359: F, t15362: F, t15366: F, t15372: F, t15376: F, t15378: F, t15382: F, t15384: F, t1901: F) -> (F,) {
    let t15388 = t15385 * t15387;
    let t15391 = t4145 * t2413;
    let t15392 = t2874 * t15391;
    let t15395 = t4145 * t2405;
    let t15396 = t10479 * t15395;
    let t15400 = 4.0 / 81.0 * t8392 * t4142;
    let t15401 = 2.0 / 9.0 * t1901 * t15346 - 2.0 / 27.0 * t1901 * t15350 + 2.0 / 9.0 * t1901 * t15355 - 2.0 / 3.0 * t1901 * t15359 + 4.0 / 9.0 * t1901 * t15362 - 4.0 / 27.0 * t1901 * t15366 - 4.0 / 3.0 * t1901 * t15372 - t15376 + 2.0 / 27.0 * t1901 * t15378 - t15382 - t15384 - 10.0 / 81.0 * t1901 * t15388 + t1901 * t15392 / 9.0 + 2.0 / 27.0 * t1901 * t15396 + t15400;
    (t15401,)
}
