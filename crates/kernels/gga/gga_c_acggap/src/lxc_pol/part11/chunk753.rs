//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 753/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk753<F: Float>(t7380: F, t8875: F, t1983: F, t513: F, t2095: F, t2318: F, t7440: F, t1323: F, t142: F, t7436: F, t128: F, t569: F, t568: F) -> (F, F, F, F, F, F, F, F) {
    let t8876 = t7380 * t8875;
    let t8878 = t1983 * t513;
    let t8879 = t2095 * t8878;
    let t8882 = t7440 * t2318;
    let t8884 = t142 * t1323;
    let t8885 = t7436 * t8884;
    let t8887 = t569 * t128;
    let t8888 = t568 * t8887;
    (t8876, t8878, t8879, t8882, t8884, t8885, t8887, t8888)
}
