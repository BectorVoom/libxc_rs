//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1293/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1293<F: Float>(t3071: F, t560: F, t20298: F, t6086: F, t20305: F, t29837: F, t19883: F, t9243: F, t113: F, t28335: F, t6085: F, t28390: F, t2155: F, t30320: F, t8077: F, t30304: F) -> (F, F, F, F, F, F, F) {
    let t30628 = t3071 * t560;
    let t30630 = t20298 * t6086 * t30628;
    let t30633 = t20305 * t6086 * t29837;
    let t30635 = t19883 * t9243;
    let t30637 = t28335 * t113;
    let t30639 = t6085 * t6086 * t30637;
    let t30643 = t28390 * t113;
    let t30645 = t6085 * t6086 * t30643;
    let t30648 = t2155 * t8077 * t30320;
    let t30651 = t2155 * t8077 * t30304;
    (t30630, t30633, t30635, t30639, t30645, t30648, t30651)
}
