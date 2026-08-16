//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 312/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk312<F: Float>(t2843: F, t4181: F, t296: F, t1255: F, t684: F, t835: F, t1234: F, t2755: F, t856: F, t91: F, t1228: F, t1775: F) -> (F, F, F, F) {
    let t4182 = t2843 * t4181;
    let t4183 = t296 * t4182;
    let t4188 = t835 * t1255 * t684;
    let t4191 = t2755 * t1234;
    let t4193 = t91 * t4191 * t856;
    let t4197 = t1775 * t1228;
    (t4183, t4188, t4193, t4197)
}
