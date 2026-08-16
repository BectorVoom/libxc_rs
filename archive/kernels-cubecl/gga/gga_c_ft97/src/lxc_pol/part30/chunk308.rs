//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 308/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk308<F: Float>(t4150: F, t684: F, t2881: F, t1250: F, t1882: F, t319: F, t3746: F, t835: F, t1212: F, t824: F) -> (F, F, F, F, F) {
    let t4151 = t4150 * t684;
    let t4152 = t2881 * t4151;
    let t4156 = t1882 * t1250;
    let t4159 = t835 * t319 * t3746;
    let t4162 = t1212 * t824;
    (t4151, t4152, t4156, t4159, t4162)
}
