//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 563/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk563<F: Float>(t1801: F, t5063: F, t5062: F, t1869: F, t1757: F, t1894: F) -> (F, F, F, F) {
    let t5064 = t1801 * t5063;
    let t5065 = t5062 * t5064;
    let t5066 = t1869 * t5065;
    let t5068 = t1894 * t1757;
    (t5064, t5065, t5066, t5068)
}
