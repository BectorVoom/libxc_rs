//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 623/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk623<F: Float>(t3894: F, t8392: F, t3877: F, t3882: F, t3888: F, t1882: F, t3979: F, t13746: F, t13753: F, t13780: F, t13794: F, t13809: F, t13811: F, t3861: F, t3866: F, t3844: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13933 = 4.0 / 81.0 * t8392 * t3894;
    let t13959 = 2.0 / 27.0 * t8392 * t3877;
    let t13961 = 2.0 / 27.0 * t8392 * t3882;
    let t13963 = 4.0 / 27.0 * t8392 * t3888;
    let t13965 = 2.0 / 9.0 * t1882 * t3979;
    let t13983 = 4.0 / 3.0 * t13746;
    let t13984 = 2.0 / 3.0 * t13753;
    let t13993 = 2.0 / 9.0 * t13780;
    let t13998 = 4.0 / 27.0 * t13794;
    let t14004 = 2.0 / 9.0 * t13809;
    let t14005 = 4.0 / 9.0 * t13811;
    let t14018 = 2.0 / 9.0 * t1882 * t3861;
    let t14020 = 4.0 / 9.0 * t1882 * t3866;
    let t14052 = 2.0 / 9.0 * t1882 * t3844;
    (t13933, t13959, t13961, t13963, t13965, t13983, t13984, t13993, t13998, t14004, t14005, t14018, t14020, t14052)
}
