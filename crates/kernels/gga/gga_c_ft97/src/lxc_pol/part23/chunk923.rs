//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 923/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk923<F: Float>(t14127: F, t28368: F, t11593: F, t1901: F, t24742: F, t24757: F, t28326: F, t28330: F, t28334: F, t28338: F, t28341: F, t28346: F, t28350: F, t28353: F, t28357: F, t28361: F, t28365: F, t446: F) -> (F, F) {
    let t28369 = t14127 * t28368;
    let t28372 = 2.0 / 3.0 * t446 * t28326 - t446 * t28330 / 9.0 + t446 * t28334 / 3.0 - t24742 / 27.0 - 2.0 / 9.0 * t28338 - t24757 - 2.0 / 9.0 * t1901 * t28341 + 2.0 / 27.0 * t1901 * t28346 - t1901 * t28350 / 9.0 - t28353 / 27.0 + t1901 * t28357 / 9.0 - 2.0 / 9.0 * t11593 * t28361 - 2.0 / 3.0 * t1901 * t28365 - 2.0 / 3.0 * t1901 * t28369;
    (t28369, t28372)
}
