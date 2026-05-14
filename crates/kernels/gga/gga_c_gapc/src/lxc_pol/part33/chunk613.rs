//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 613/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk613<F: Float>(t4978: F, t676: F, t1798: F, t618: F, t144: F, t1975: F, t1453: F, t190: F, t1303: F, t1672: F, t1839: F, t674: F, t1509: F, t193: F, t670: F, t22: F) -> (F, F, F, F, F, F, F, F) {
    let t4979 = t676 * t4978;
    let t4991 = t618 * t1798;
    let t4995 = t1975 * t144;
    let t5011 = t190 * t1453;
    let t5017 = t1672 * t1303;
    let t5021 = t674 * t1839;
    let t5022 = t5021 * t1509;
    let t5054 = t670 * t193;
    let t5056 = 1.0 / t22 / t5054;
    (t4979, t4991, t4995, t5011, t5017, t5022, t5054, t5056)
}
