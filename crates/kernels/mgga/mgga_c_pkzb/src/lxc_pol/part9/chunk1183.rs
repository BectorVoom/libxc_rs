//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1183/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1183<F: Float>(t1676: F, t7177: F, t16825: F, t16938: F, t16946: F, t16950: F, t20371: F, t20373: F, t20375: F, t20376: F, t20377: F, t20379: F, t2718: F, t5191: F, t6758: F) -> (F, F) {
    let t20615 = t7177 * t1676;
    let t20623 = F::new(18.0) * t2718 * t5191 * t6758 + t16825 + t16938 + t16946 + t16950 + t20371 + t20373 - t20375 - t20376 + t20377 + t20379;
    (t20615, t20623)
}
