//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 938/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk938<F: Float>(t12270: F, t2592: F, t42517: F, t13765: F, t4342: F, t1382: F, t2497: F, t3718: F, t13914: F, t1960: F, t42513: F, t44207: F, t44208: F, t44211: F, t44215: F, t44217: F, t47110: F, t841: F) -> (F, F, F, F) {
    let t47112 = t2592 * t12270;
    let t47113 = 2.0 * t42517;
    let t47114 = t4342 * t13765;
    let t47115 = 2.0 * t47114;
    let t47120 = t1382 * t3718 * t2497;
    let t47121 = 2.0 * t47120;
    let t47124 = 2.0 * t13914 * t1960 * t841 + t42513 - t44207 - t44208 + 2.0 * t44211 + 2.0 * t44215 + 2.0 * t44217 - t47110 - t47112 - t47113 - t47115 - t47121;
    (t47113, t47115, t47121, t47124)
}
