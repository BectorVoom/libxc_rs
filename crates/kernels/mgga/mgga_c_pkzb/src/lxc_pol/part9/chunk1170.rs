//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1170/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1170<F: Float>(t16940: F, t1542: F, t2607: F, t16810: F, t16813: F, t16822: F, t16825: F, t16938: F, t16946: F, t16950: F, t20363: F, t20365: F, t20367: F, t20369: F, t20371: F, t20373: F, t20375: F, t20376: F) -> (F, F, F) {
    let t20377 = F::new(192.0) * t16940;
    let t20378 = t1542 * t2607;
    let t20379 = F::new(60.0) * t20378;
    let t20380 = -t20363 + t16810 - t16813 - t16822 + t20365 - t20367 - t20369 + t20371 + t20373 - t20375 + t16825 - t20376 + t16938 + t20377 + t16946 + t16950 + t20379;
    (t20377, t20379, t20380)
}
