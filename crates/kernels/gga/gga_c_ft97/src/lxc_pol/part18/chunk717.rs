//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 717/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk717<F: Float>(t12318: F, t446: F, t11050: F, t569: F, t11059: F, t2205: F, t1882: F, t3339: F, t3408: F, t358: F, t363: F, t1969: F, t2223: F, t3337: F) -> (F, F, F, F, F, F, F, F) {
    let t12319 = t446 * t12318;
    let t12321 = t569 * t11050;
    let t12322 = t446 * t12321;
    let t12324 = t2205 * t11059;
    let t12325 = t446 * t12324;
    let t12327 = t1882 * t3339;
    let t12328 = t12327 / 27.0;
    let t12329 = t3408 * t358;
    let t12330 = t12329 * t363;
    let t12331 = t1969 * t12330;
    let t12332 = t446 * t12331;
    let t12334 = t3337 * t2223;
    (t12319, t12322, t12325, t12327, t12328, t12330, t12332, t12334)
}
