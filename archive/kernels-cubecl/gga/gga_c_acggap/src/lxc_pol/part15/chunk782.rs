//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 782/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk782<F: Float>(t1503: F, t2041: F, t1165: F, t1411: F, t604: F, t2068: F, t495: F, t7381: F, t7380: F, t1983: F, t513: F, t2095: F) -> (F, F, F, F, F, F, F) {
    let t8866 = t2041 * t1503;
    let t8869 = t1165 * t604 * t1411;
    let t8870 = t2068 * t8869;
    let t8875 = t7381 * t495;
    let t8876 = t7380 * t8875;
    let t8878 = t1983 * t513;
    let t8879 = t2095 * t8878;
    (t8866, t8869, t8870, t8875, t8876, t8878, t8879)
}
