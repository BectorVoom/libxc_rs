//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1232/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1232<F: Float>(t1622: F, t35251: F, t1633: F, t11420: F, t116: F, t34021: F, t11391: F, t677: F, t11412: F, t169: F, t4043: F, t8960: F) -> (F, F, F, F, F) {
    let t35252 = t1622 * t35251;
    let t35254 = t1633 * t35251;
    let t35257 = t116 * t34021 * t11420;
    let t35259 = t11391 * t677;
    let t35263 = t169 * t11412 * t4043 * t8960;
    (t35252, t35254, t35257, t35259, t35263)
}
