//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1872/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1872<F: Float>(t12369: F, t19743: F, t22633: F, t22897: F, t562: F, t6330: F, t1307: F, t26446: F, t90591: F, t1992: F, t20018: F, t6976: F) -> (F, F, F, F) {
    let t97007 = t22633 * t22897 * t19743 * t12369;
    let t97011 = t562 * t6330;
    let t97014 = t90591 * t26446 * t97011 * t1307;
    let t97017 = t1992 * t6976 * t20018;
    (t97007, t97011, t97014, t97017)
}
