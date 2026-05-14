//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1071/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1071<F: Float>(t1762: F, t4708: F, t4807: F, t378: F, t4990: F, t735: F, t1751: F, t4973: F, t1398: F, t1411: F, t410: F, t4694: F, t1485: F, t18938: F, t4868: F) -> (F, F, F, F, F, F) {
    let t18979 = 0.1301229756036208781e0 * t1762 * t4807 * t4708;
    let t18984 = 0.1301229756036208781e0 * t735 * t378 * t4990;
    let t18986 = t1751 * t4973;
    let t18990 = 0.43374325201206959368e-1 * t735 * t1398 * t1411;
    let t18991 = t410 * t4694;
    let t18995 = 0.57895126195293126241e3 * t4868 * t18938 * t1485;
    (t18979, t18984, t18986, t18990, t18991, t18995)
}
