//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 773/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk773<F: Float>(t2294: F, t2568: F, t2139: F, t2578: F, t2553: F, t6118: F, t2195: F, t2666: F, t6343: F, t938: F, t551: F, t549: F) -> (F, F, F, F, F, F) {
    let t7360 = t2294 * t2568;
    let t7362 = F::new(0.69345773920434148506e0) * t2139 * t7360;
    let t7365 = t2294 * t2578;
    let t7367 = F::new(0.69345773920434148506e0) * t2139 * t7365;
    let t7377 = F::new(0.25610080155860322884e0) * t6118 * t2553;
    let t7383 = t2195 * t2666;
    let t7386 = t6343 * t938;
    let t7387 = t551 * t7386;
    let t7388 = t549 * t7387;
    (t7362, t7367, t7377, t7383, t7386, t7388)
}
