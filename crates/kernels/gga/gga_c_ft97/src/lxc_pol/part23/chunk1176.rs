//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1176/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1176<F: Float>(t10580: F, t1495: F, t1882: F, t29178: F, t7098: F, t8232: F, t56456: F, t6273: F, t25271: F, t310: F, t29304: F, t29321: F, t29325: F, t29124: F, t8392: F, t29281: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t113903 = t10580 * t1495;
    let t113914 = 2.0 / 9.0 * t1882 * t29178;
    let t113915 = t8232 * t7098;
    let t113932 = t56456 * t6273;
    let t113939 = t310 * t25271;
    let t113956 = 2.0 / 9.0 * t1882 * t29304;
    let t113968 = 4.0 / 9.0 * t1882 * t29321;
    let t113972 = 4.0 / 9.0 * t1882 * t29325;
    let t113974 = 4.0 / 9.0 * t8392 * t29124;
    let t113980 = 2.0 / 9.0 * t1882 * t29281;
    (t113903, t113914, t113915, t113932, t113939, t113956, t113968, t113972, t113974, t113980)
}
