//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 893/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk893<F: Float>(t7494: F, t9807: F, t46005: F, t739: F, t7577: F, t2305: F, t39284: F, t1550: F, t2060: F, t30453: F, t30311: F, t903: F, t2604: F, t9957: F, t1707: F, t2064: F, t3928: F) -> (F, F, F, F, F, F, F) {
    let t47367 = t7494 * t9807;
    let t47371 = t739 * t7577 * t46005;
    let t47375 = t39284 * t2305;
    let t47378 = t1550 * t2060 * t30453;
    let t47381 = t903 * t2060 * t30311;
    let t47385 = t2604 * t9957;
    let t47390 = t3928 * t2064 * t1707;
    (t47367, t47371, t47375, t47378, t47381, t47385, t47390)
}
