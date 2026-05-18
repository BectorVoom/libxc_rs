//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 909/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk909<F: Float>(t1160: F, t676: F, t2372: F, t2568: F, t1240: F, t2681: F, t1200: F, t7606: F, t19106: F, t800: F, t4092: F, t2843: F) -> (F, F, F, F, F, F, F) {
    let t67847 = t676 * t1160;
    let t67996 = t2372 * t2568;
    let t69996 = t2681 * t1240;
    let t70497 = t1200 * t7606;
    let t70550 = t800 * t19106;
    let t70779 = t4092 * t19106;
    let t72190 = t2681 * t2843;
    (t67847, t67996, t69996, t70497, t70550, t70779, t72190)
}
