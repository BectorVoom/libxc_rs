//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1385/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1385<F: Float>(t22820: F, t25697: F, t6086: F, t25466: F, t6535: F, t113: F, t2526: F, t494: F, t2155: F, t6063: F, t545: F, t7600: F, t6088: F, t146: F, t6091: F, t978: F) -> (F, F, F, F, F, F) {
    let t26268 = t22820 * t6086 * t25697;
    let t26271 = t6535 * t6086 * t25466;
    let t26274 = t2526 * t494 * t113;
    let t26276 = t2155 * t6063 * t26274;
    let t26278 = t545 * t7600;
    let t26279 = t26278 * t6088;
    let t26282 = t146 * t6091 * t978;
    (t26268, t26271, t26274, t26276, t26279, t26282)
}
