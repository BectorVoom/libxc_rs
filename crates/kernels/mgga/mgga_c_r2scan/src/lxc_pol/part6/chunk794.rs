//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 794/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk794<F: Float>(t1678: F, t607: F, t159: F, t1686: F, t1696: F, t745: F, t1745: F, t732: F, t1731: F, t5311: F) -> (F, F, F, F, F, F) {
    let t5407 = t607 * t1678;
    let t5408 = t159 * t5407;
    let t5409 = t5408 * t1686;
    let t5411 = t1696 * t745;
    let t5413 = t732 * t1745;
    let t5416 = t1731 * t5311;
    (t5407, t5408, t5409, t5411, t5413, t5416)
}
