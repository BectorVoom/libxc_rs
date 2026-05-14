//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 591/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk591<F: Float>(t1550: F, t8404: F, t2060: F, t5267: F, t903: F, t1627: F, t645: F, t3928: F, t5888: F, t7577: F, t739: F, t1469: F, t236: F, t1971: F, t7365: F, t1475: F, t498: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8405 = t1550 * t8404;
    let t8407 = t2060 * t5267;
    let t8408 = t903 * t8407;
    let t8410 = t645 * t1627;
    let t8411 = t3928 * t8410;
    let t8413 = t7577 * t5888;
    let t8414 = t739 * t8413;
    let t8416 = t236 * t1469;
    let t8417 = t1971 * t8416;
    let t8418 = t7365 * t8417;
    let t8420 = t1475 * t498;
    (t8405, t8407, t8408, t8410, t8411, t8413, t8414, t8417, t8418, t8420)
}
