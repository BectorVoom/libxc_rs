//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 761/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk761<F: Float>(t171: F, t5397: F, t5398: F, t21: F, t502: F, t1684: F, t1680: F, t1678: F, t607: F, t159: F, t1686: F, t1745: F, t732: F, t1731: F, t5311: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5401 = 0.6858336e0 * t5397 * t171 * t5398;
    let t5402 = t21 * t502;
    let t5403 = t1684 * t5402;
    let t5405 = 0.16936279733333333332e-2 * t1680 * t5403;
    let t5407 = t607 * t1678;
    let t5408 = t159 * t5407;
    let t5409 = t5408 * t1686;
    let t5413 = t732 * t1745;
    let t5416 = t1731 * t5311;
    (t5401, t5402, t5403, t5405, t5407, t5408, t5409, t5413, t5416)
}
