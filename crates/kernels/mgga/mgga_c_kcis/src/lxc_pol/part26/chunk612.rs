//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 612/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk612<F: Float>(t4160: F, t6905: F, t1889: F, t5632: F, t1395: F, t1394: F, t1444: F, t6281: F) -> (F, F, F, F, F) {
    let t6906 = t4160 * t6905;
    let t6908 = t5632 * t1889;
    let t6909 = t1395 * t6908;
    let t6910 = t1394 * t6909;
    let t6912 = t1444 * t6281;
    (t6906, t6908, t6909, t6910, t6912)
}
