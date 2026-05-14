//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 500/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk500<F: Float>(t154: F, t2048: F, t655: F, t276: F, t1843: F, t742: F, t1419: F, t275: F) -> (F, F, F, F) {
    let t2050 = t154 * t2048 * t655;
    let t2051 = t276 * t2050;
    let t2054 = t154 * t742 * t1843;
    let t2057 = t1419 * t275;
    (t2050, t2051, t2054, t2057)
}
