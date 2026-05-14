//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 566/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk566<F: Float>(t39: F, t550: F, t133: F, t542: F, t548: F, t135: F, t8078: F, t40: F, t6: F, t12: F, t171: F, t341: F, t630: F, t343: F, t70: F, t120: F, t358: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8851 = t550 * t39;
    let t8852 = t133 * t8851;
    let t8859 = t542 * t8851;
    let t8906 = t548 * t548;
    let t8907 = 1.0 / t8906;
    let t8908 = t135 * t8907;
    let t8914 = 0.18521666970164609055e-1 * t8078;
    let t8946 = t6 / t40;
    let t8947 = t12 * t171;
    let t8948 = t8946 * t8947;
    let t8959 = t341 * t630;
    let t8963 = t341 * t343 * t70;
    let t8965 = t120 * t358;
    (t8851, t8852, t8859, t8906, t8907, t8908, t8914, t8948, t8959, t8963, t8965)
}
