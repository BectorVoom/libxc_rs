//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 694/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk694<F: Float>(t1647: F, t447: F, t986: F, t1882: F, t3210: F, t8232: F, t951: F, t1755: F, t452: F, t3216: F, t3291: F, t432: F, t1786: F, t971: F) -> (F, F, F, F, F, F, F) {
    let t11878 = t447 * t986 * t1647;
    let t11882 = 2.0 / 27.0 * t1882 * t3210;
    let t11883 = t8232 * t951;
    let t11887 = t452 * t986 * t1755;
    let t11897 = 2.0 / 9.0 * t1882 * t3216;
    let t11899 = t452 * t3291 * t432;
    let t11902 = t1786 * t971;
    (t11878, t11882, t11883, t11887, t11897, t11899, t11902)
}
