//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 655/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk655<F: Float>(t14213: F, t2574: F, t265: F, t3746: F, t724: F, t773: F, t1882: F, t3839: F, t1140: F, t8232: F, t1131: F, t2569: F, t2568: F, t729: F, t3848: F, t1170: F) -> (F, F, F, F, F, F, F, F) {
    let t14215 = t2574 * t265 * t14213;
    let t14219 = t724 * t773 * t3746;
    let t14223 = 4.0 / 9.0 * t1882 * t3839;
    let t14224 = t8232 * t1140;
    let t14226 = t1131 * t2569;
    let t14228 = t729 * t2568 * t14226;
    let t14232 = 2.0 / 27.0 * t1882 * t3848;
    let t14233 = t8232 * t1170;
    (t14215, t14219, t14223, t14224, t14226, t14228, t14232, t14233)
}
