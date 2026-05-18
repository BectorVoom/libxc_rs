//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 767/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk767<F: Float>(t1137: F, t1805: F, t1140: F, t1809: F, t1801: F, t1797: F, t1750: F, t3431: F, t174: F, t1814: F) -> (F, F, F, F, F, F) {
    let t5842 = t1137 * t1805;
    let t5844 = t1140 * t1809;
    let t5846 = t1137 * t1801;
    let t5848 = t1140 * t1797;
    let t5850 = t3431 * t1750;
    let t5852 = t174 * t1814;
    (t5842, t5844, t5846, t5848, t5850, t5852)
}
