//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 912/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk912<F: Float>(t23884: F, t586: F, t1369: F, t28: F, t376: F, t5909: F, t1359: F, t1570: F, t1559: F, t1969: F, t446: F, t1370: F, t1637: F, t358: F, t5842: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23885 = t586 * t23884;
    let t23887 = t1369 * t28 * t23885;
    let t23890 = t1369 * t376 * t5909;
    let t23892 = t1359 * t1570;
    let t23893 = t23892 * t1559;
    let t23894 = t1969 * t23893;
    let t23895 = t446 * t23894;
    let t23898 = t1369 * t1637 * t1370;
    let t23899 = 2.0 / 9.0 * t23898;
    let t23900 = t5842 * t358;
    (t23885, t23887, t23890, t23892, t23894, t23895, t23898, t23899, t23900)
}
