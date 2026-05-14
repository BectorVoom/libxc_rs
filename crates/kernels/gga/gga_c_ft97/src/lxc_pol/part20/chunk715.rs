//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 715/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk715<F: Float>(t4181: F, t684: F, t15312: F, t1221: F, t8232: F, t15134: F, t296: F, t15140: F, t1242: F, t2399: F, t89: F, t1882: F, t4276: F, t4280: F, t10443: F, t4146: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15313 = t4181 * t684;
    let t15314 = t15312 * t15313;
    let t15318 = t8232 * t1221;
    let t15322 = t296 * t15134;
    let t15325 = t296 * t15140;
    let t15329 = t89 * t2399 * t1242;
    let t15334 = 2.0 / 9.0 * t1882 * t4276;
    let t15336 = 2.0 / 9.0 * t1882 * t4280;
    let t15338 = t10443 * t4146;
    (t15313, t15314, t15318, t15322, t15325, t15329, t15334, t15336, t15338)
}
