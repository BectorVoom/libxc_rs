//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1176/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1176<F: Float>(t19957: F, t19991: F, t20032: F, t20106: F, t20163: F, t20223: F, t20404: F, t20438: F, t5389: F, t621: F, t1044: F, t5373: F) -> (F, F, F) {
    let t20441 = t19957 + t19991 + t20032 + t20106 + t20163 + t20223 + t20404 + t20438;
    let t20474 = t5389 * t621;
    let t20482 = t1044 * t5373;
    (t20441, t20474, t20482)
}
