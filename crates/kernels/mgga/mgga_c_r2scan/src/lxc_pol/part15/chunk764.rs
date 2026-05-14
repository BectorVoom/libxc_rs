//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 764/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk764<F: Float>(t2294: F, t2578: F, t2139: F, t1570: F, t2567: F, t360: F, t1551: F, t2124: F, t2545: F, t2553: F, t6118: F, t113: F, t1543: F, t2572: F, t2195: F, t2666: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7365 = t2294 * t2578;
    let t7367 = 0.69345773920434148506e0 * t2139 * t7365;
    let t7368 = t2567 * t1570;
    let t7369 = t360 * t7368;
    let t7373 = t2124 * t2545 * t1551;
    let t7377 = 0.25610080155860322884e0 * t6118 * t2553;
    let t7378 = t113 * t1543;
    let t7379 = t2572 * t7378;
    let t7380 = t360 * t7379;
    let t7383 = t2195 * t2666;
    (t7367, t7368, t7369, t7373, t7377, t7378, t7379, t7380, t7383)
}
