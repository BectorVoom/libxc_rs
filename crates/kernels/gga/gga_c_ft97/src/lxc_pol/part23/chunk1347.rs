//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1347/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1347<F: Float>(t24980: F, t24981: F, t4635: F, t6334: F, t856: F, t1486: F, t31552: F, t681: F, t1882: F, t31375: F, t2665: F, t446: F, t4917: F, t99511: F, t16579: F, t10409: F, t99363: F) -> (F, F, F, F, F, F, F, F) {
    let t126877 = t24980 * t24981 * t6334 * t4635 * t856;
    let t126880 = t1486 * t681 * t31552;
    let t126881 = t126880 / 6.0;
    let t126882 = t1882 * t31375;
    let t126883 = 2.0 / 9.0 * t126882;
    let t126886 = t446 * t2665 * t99511 * t4917;
    let t126890 = t446 * t2665 * t6334 * t16579;
    let t126894 = t446 * t10409 * t99363 * t4917;
    (t126877, t126880, t126881, t126882, t126883, t126886, t126890, t126894)
}
