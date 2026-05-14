//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1411/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1411<F: Float>(t1882: F, t31752: F, t31791: F, t31724: F, t8392: F, t31740: F, t31895: F, t114935: F, t114938: F, t114940: F, t114942: F, t114979: F, t11593: F, t125705: F, t1901: F, t19513: F, t19586: F, t24886: F, t25368: F, t2881: F, t29093: F, t296: F, t446: F, t4969: F, t6393: F, t835: F) -> (F,) {
    let t128542 = t1882 * t31752;
    let t128544 = t1882 * t31791;
    let t128549 = t8392 * t31724;
    let t128551 = t8392 * t31740;
    let t128565 = t8392 * t31895;
    let t128567 = 8.0 / 27.0 * t11593 * t29093 * t19513 + t114935 + t114938 + t114940 - t114942 - 2.0 / 9.0 * t128542 + t128544 / 9.0 + 2.0 / 3.0 * t446 * t296 * t125705 - 2.0 / 27.0 * t128549 - t128551 / 27.0 - 2.0 / 9.0 * t1901 * t24886 * t19586 + 8.0 / 27.0 * t114979 - 2.0 / 9.0 * t1901 * t2881 * t25368 * t4969 + 2.0 / 9.0 * t446 * t835 * t6393 * t4969 + 4.0 / 9.0 * t128565;
    (t128567,)
}
