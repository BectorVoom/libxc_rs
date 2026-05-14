//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1254/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1254<F: Float>(t159: F, t1678: F, t1686: F, t3128: F, t1669: F, t8916: F, t1759: F, t584: F, t8915: F, t1748: F, t8908: F, t3034: F, t625: F, t1764: F, t5326: F, t8987: F) -> (F, F, F, F, F, F, F) {
    let t28469 = t159 * t3128 * t1678 * t1686;
    let t28471 = t8916 * t1669;
    let t28476 = t584 * t8915 * t1759;
    let t28479 = t8908 * t1748;
    let t28494 = t3034 * t625;
    let t28495 = t28494 * t1764;
    let t28497 = t8987 * t5326;
    (t28469, t28471, t28476, t28479, t28494, t28495, t28497)
}
