//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 690/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk690<F: Float>(t1595: F, t2321: F, t882: F, t1352: F, t875: F, t535: F, t3811: F, t883: F, t2325: F, t161: F, t2366: F, t1529: F) -> (F, F, F, F, F, F) {
    let t6459 = t1595 * t2321;
    let t6460 = t882 * t6459;
    let t6462 = t875 * t1352;
    let t6463 = t535 * t6462;
    let t6466 = t883 * t3811;
    let t6467 = t2325 * t6466;
    let t6468 = t882 * t6467;
    let t6470 = t161 * t2366;
    let t6471 = t1529 * t6470;
    (t6460, t6463, t6466, t6468, t6470, t6471)
}
