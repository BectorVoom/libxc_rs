//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1012/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1012<F: Float>(t1769: F, t8794: F, t10798: F, t8797: F, t5013: F, t1785: F, t8518: F, t5015: F, t10902: F, t8746: F, t10906: F, t1744: F, t4928: F, t8763: F, t7157: F, t10913: F) -> (F, F, F, F, F, F) {
    let t23413 = t8794 * t1769;
    let t23415 = t10798 * t8797;
    let t23416 = t5013 * t23415;
    let t23420 = t8518 * t1785;
    let t23421 = t5015 * t23420;
    let t23424 = t10902 * t8746;
    let t23425 = t10906 * t1744;
    let t23426 = t23424 * t23425;
    let t23429 = t4928 * t8763;
    let t23430 = t23429 * t7157;
    let t23433 = t10913 * t8746;
    (t23413, t23416, t23421, t23426, t23430, t23433)
}
