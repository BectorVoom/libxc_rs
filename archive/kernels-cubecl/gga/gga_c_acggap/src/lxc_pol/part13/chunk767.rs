//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 767/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk767<F: Float>(t464: F, t7973: F, t2122: F, t315: F, t323: F, t309: F, t2132: F, t2131: F, t322: F, t2138: F, t7911: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7974 = t7973 * t464;
    let t7976 = t315 * t2122;
    let t7977 = t7976 * t323;
    let t7979 = t2122 * t309;
    let t7980 = t2132 * t7979;
    let t7981 = t2131 * t7980;
    let t7983 = t2122 * t322;
    let t7984 = t2132 * t7983;
    let t7985 = t2138 * t7984;
    let t7987 = t315 * t7911;
    (t7974, t7976, t7977, t7979, t7980, t7981, t7983, t7984, t7985, t7987)
}
