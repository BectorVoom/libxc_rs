//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 359/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk359<F: Float>(t241: F, t258: F, t3951: F, t1162: F, t681: F, t89: F, t2338: F, t2341: F, t2518: F, t3688: F, t3693: F, t3697: F, t3702: F, t3707: F, t3710: F, t3715: F, t3720: F, t3824: F, t3904: F, t3940: F) -> (F, F, F) {
    let t3953 = t241 * t3951 * t258;
    let t3958 = t89 * t681 * t1162;
    let t3972 = -t3904 / 4.0 + t3940 / 2.0 + t2518 + t2338 / 9.0 + t2341 / 3.0 + t3688 / 9.0 - 2.0 / 9.0 * t3693 + t3697 / 3.0 + 2.0 / 3.0 * t3702 + 2.0 / 3.0 * t3707 + t3710 / 3.0 + t3715 / 3.0 + 2.0 * t3720 - t3824;
    (t3953, t3958, t3972)
}
