//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1246/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1246<F: Float>(t16193: F, t16230: F, t16273: F, t16275: F, t16280: F, t16283: F, t16287: F, t16290: F, t16481: F, t16486: F, t23906: F, t23907: F, t23908: F, t23909: F, t23916: F, t23917: F, t23921: F) -> (F,) {
    let t24524 = -t16193 - t16230 - t16273 + t16275 - t23906 + t23907 + t23908 + t23909 - t16280 + t16283 + t16287 - t16290 - t23916 - t23917 + t23921 + t16481 - t16486;
    (t24524,)
}
