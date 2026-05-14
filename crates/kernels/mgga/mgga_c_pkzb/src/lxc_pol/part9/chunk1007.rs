//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1007/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1007<F: Float>(t1835: F, t87: F, t1971: F, t5493: F, t5762: F, t713: F, t1908: F, t1915: F, t5829: F, t690: F, t1731: F, t218: F, t220: F, t5555: F, t679: F, t1878: F, t1885: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17359 = t1835 * t1835;
    let t17361 = 1.0 / t87 / t17359;
    let t17381 = t1971 * t5493;
    let t17385 = t5762 * t713;
    let t17388 = t1908 * t1915;
    let t17391 = t690 * t5829;
    let t17402 = t218 * t1731 * t220;
    let t17403 = 0.13490888888888888889e1 * t17402;
    let t17405 = t218 * t5555 * t679;
    let t17408 = t218 * t1878 * t1885;
    (t17361, t17381, t17385, t17388, t17391, t17402, t17403, t17405, t17408)
}
