//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 826/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk826<F: Float>(t25188: F, t2844: F, t296: F, t1503: F, t8232: F, t1882: F, t6355: F, t1508: F, t2682: F, t2862: F, t2739: F, t840: F, t6393: F, t824: F, t684: F, t835: F) -> (F, F, F, F, F, F, F, F) {
    let t25189 = t25188 * t2844;
    let t25190 = t296 * t25189;
    let t25194 = 4.0 / 27.0 * t8232 * t1503;
    let t25195 = t1882 * t6355;
    let t25198 = t2862 * t1508 * t2682;
    let t25202 = t840 * t1508 * t2739;
    let t25206 = t840 * t6393 * t824;
    let t25210 = t835 * t6393 * t684;
    (t25189, t25190, t25194, t25195, t25198, t25202, t25206, t25210)
}
