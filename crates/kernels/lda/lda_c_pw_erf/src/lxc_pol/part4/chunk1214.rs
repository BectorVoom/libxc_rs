//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1214/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1214<F: Float>(t17983: F, t568: F, t6611: F, t13749: F, t1325: F, t197: F, t2176: F, t504: F, t784: F, t1339: F, t519: F, t739: F, t10409: F, t10412: F, t10414: F, t10417: F, t17968: F, t17970: F, t17972: F, t17975: F, t17976: F, t17978: F, t17980: F, t17982: F) -> (F, F, F, F, F, F) {
    let t17984 = 16.0 / 45.0 * t17983;
    let t17985 = t6611 * t568;
    let t17986 = 16.0 / 45.0 * t17985;
    let t17987 = 32.0 / 135.0 * t13749;
    let t17992 = 32.0 / 45.0 * t1325 * t2176 * t197 * t784 * t504;
    let t17996 = 32.0 / 45.0 * t519 * t2176 * t1339 * t739;
    let t17997 = -t17968 - t17970 - t17972 + 8.0 * t10409 + t10412 + 0.002206740740740741 * t10414 + t10417 + t17975 - t17976 + t17978 - t17980 - t17982 + t17984 + t17986 - t17987 + t17992 - t17996;
    (t17984, t17986, t17987, t17992, t17996, t17997)
}
