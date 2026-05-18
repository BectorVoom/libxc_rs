//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 565/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk565<F: Float>(t3325: F, t3330: F, t2660: F, t3045: F, t2767: F, t129: F, t2619: F, t197: F, t2621: F, t2712: F, t1077: F, t1936: F, t916: F) -> (F, F, F, F, F, F, F, F) {
    let t3331 = t3325 * t3330;
    let t3333 = t2660 * t3045;
    let t3334 = t3333 * t2767;
    let t3336 = t2619 * t129;
    let t3337 = t197 * t2621;
    let t3338 = t3336 * t3337;
    let t3340 = t197 * t2712;
    let t3341 = t1077 * t3340;
    let t3343 = t916 * t1936;
    (t3331, t3334, t3336, t3337, t3338, t3340, t3341, t3343)
}
