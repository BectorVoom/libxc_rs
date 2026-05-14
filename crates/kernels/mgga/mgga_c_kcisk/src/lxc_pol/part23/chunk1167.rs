//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1167/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1167<F: Float>(t2737: F, t32422: F, t32401: F, t32047: F, t32050: F, t32060: F, t32063: F, t32076: F, t32079: F, t32082: F, t32120: F, t32124: F, t32380: F, t32399: F, t32402: F, t32417: F, t9512: F, t9516: F, t9519: F, t9544: F) -> (F, F, F) {
    let t32423 = t2737 * t32422;
    let t32425 = t2737 * t32401;
    let t32427 = -0.34722222222222222222e-2 * t32399 + 0.13402777777777777778e-2 * t32402 - 0.60312500000000000001e-2 * t9516 * t32380 - 0.23214722222222222222e-2 * t32047 + 0.15476481481481481481e-2 * t32050 + 0.10416666666666666667e-1 * t9512 * t9544 - 0.34822083333333333332e-2 * t32060 + 0.23214722222222222222e-2 * t32063 + 0.92858888888888888886e-2 * t32076 - 0.61905925925925925925e-2 * t32079 + 0.11607361111111111111e-2 * t32082 - 0.38691203703703703703e-3 * t32120 + 0.34822083333333333332e-2 * t32124 + 0.40208333333333333334e-2 * t32417 * t9519 - 0.10416666666666666667e-1 * t2737 * t32380 + 0.34722222222222222222e-2 * t32423 + 0.34722222222222222222e-2 * t32425;
    (t32423, t32425, t32427)
}
