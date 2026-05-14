//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1279/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1279<F: Float>(t1969: F, t4417: F, t446: F, t95384: F, t23900: F, t4431: F, t30232: F, t40830: F, t558: F, t1369: F, t30233: F, t376: F, t30255: F, t5890: F, t1039: F, t26768: F, t28: F, t586: F) -> (F, F, F, F, F, F, F) {
    let t119868 = t446 * t1969 * t95384 * t4417;
    let t119872 = t446 * t1969 * t23900 * t4431;
    let t119876 = t446 * t40830 * t30232 * t558;
    let t119879 = t1369 * t376 * t30233;
    let t119881 = t5890 * t376 * t30255;
    let t119882 = t119881 / 6.0;
    let t119886 = t5890 * t28 * t586 * t26768 * t1039;
    (t119868, t119872, t119876, t119879, t119881, t119882, t119886)
}
