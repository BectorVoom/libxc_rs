//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1220/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1220<F: Float>(t1306: F, t20824: F, t20827: F, t20831: F, t20892: F, t20895: F, t20898: F, t20900: F, t20902: F, t21262: F, t21265: F, t2997: F, t6058: F) -> F {
    let t21266 = -t1306 * t2997 * t6058 - t20824 - t20827 + t20831 + t20892 - t20895 + t20898 + t20900 + t20902 + t21262 - t21265;
    t21266
}
