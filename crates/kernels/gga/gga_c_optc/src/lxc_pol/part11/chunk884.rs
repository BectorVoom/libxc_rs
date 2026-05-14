//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 884/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk884<F: Float>(t17338: F, t17342: F, t17346: F, t17350: F, t17354: F, t17358: F, t17399: F, t17401: F, t17403: F, t17406: F, t17409: F, t17412: F, t17419: F, t8643: F, t17396: F, t1056: F) -> (F, F) {
    let t17421 = -t8643 - 0.59793333333333333333e0 * t17346 + 0.17938e1 * t17354 - 0.28483875e1 * t17399 + 0.46074375e0 * t17401 + 0.3071625e0 * t17403 + 0.16431333333333333333e0 * t17406 - 0.49293999999999999999e0 * t17409 - 0.36514074074074074075e-1 * t17412 - 0.33218518518518518518e0 * t17338 + 0.11958666666666666667e1 * t17342 - 0.17938e1 * t17350 - 0.29896666666666666667e0 * t17358 - 0.82156666666666666667e-1 * t17419;
    let t17422 = t17396 + t17421;
    let t17423 = t17422 * t1056;
    (t17422, t17423)
}
