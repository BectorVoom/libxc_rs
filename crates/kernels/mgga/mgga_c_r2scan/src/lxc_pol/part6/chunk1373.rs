//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1373/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1373<F: Float>(t2196: F, t2625: F, t551: F, t6343: F, t20090: F, t20820: F, t923: F, t1600: F, t8204: F, t6121: F, t7204: F, t25307: F, t538: F, t6155: F, t146: F, t6533: F, t978: F) -> (F, F, F, F, F, F) {
    let t26052 = t2196 * t551 * t6343 * t2625;
    let t26053 = 0.15256070262495512671e2 * t26052;
    let t26060 = t20090 * t923 * t20820;
    let t26062 = t1600 * t8204;
    let t26066 = t7204 * t6121;
    let t26085 = t6155 * t538 * t25307;
    let t26088 = t146 * t6533 * t978;
    (t26053, t26060, t26062, t26066, t26085, t26088)
}
