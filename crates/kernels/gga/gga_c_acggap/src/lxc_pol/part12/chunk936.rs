//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 936/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk936<F: Float>(t1089: F, t12610: F, t2079: F, t2080: F, t1967: F, t7767: F, t1459: F, t1980: F, t31024: F, t7458: F, t2117: F, t980: F) -> (F, F, F, F) {
    let t31245 = t2079 * t1089 * t12610 * t2080;
    let t31247 = t1967 * t7767;
    let t31251 = t1980 * t7458 * t1459 * t31024;
    let t31253 = t980 * t2117;
    (t31245, t31247, t31251, t31253)
}
