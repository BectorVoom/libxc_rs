//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 922/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk922<F: Float>(t46005: F, t739: F, t7577: F, t2305: F, t39284: F, t1550: F, t2060: F, t30453: F, t30311: F, t903: F, t2604: F, t9957: F, t1707: F, t2064: F, t3928: F, t6522: F, t7778: F) -> (F, F, F, F, F, F, F) {
    let t47371 = t739 * t7577 * t46005;
    let t47375 = t39284 * t2305;
    let t47378 = t1550 * t2060 * t30453;
    let t47381 = t903 * t2060 * t30311;
    let t47385 = t2604 * t9957;
    let t47390 = t3928 * t2064 * t1707;
    let t47393 = t1550 * t7778 * t6522;
    (t47371, t47375, t47378, t47381, t47385, t47390, t47393)
}
