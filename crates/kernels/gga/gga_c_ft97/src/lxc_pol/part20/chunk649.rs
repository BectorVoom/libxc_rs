//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 649/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk649<F: Float>(t13757: F, t14163: F, t3842: F, t684: F, t10007: F, t3859: F, t2568: F, t737: F) -> (F, F, F, F, F, F) {
    let t14164 = t14163 * t13757;
    let t14167 = t3842 * t684;
    let t14168 = t10007 * t14167;
    let t14171 = t3859 * t684;
    let t14172 = t10007 * t14171;
    let t14175 = t737 * t2568;
    (t14164, t14167, t14168, t14171, t14172, t14175)
}
