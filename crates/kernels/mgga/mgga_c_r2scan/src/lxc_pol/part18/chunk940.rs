//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 940/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk940<F: Float>(t2530: F, t921: F, t2182: F, t979: F, t3071: F, t6212: F, t3056: F, t560: F, t113: F, t28335: F, t28390: F, t2892: F, t146: F, t5094: F, t978: F, t3053: F, t481: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30320 = t921 * t2530;
    let t30370 = t2182 * t979;
    let t30428 = t6212 * t3071;
    let t30468 = t3056 * t560;
    let t30628 = t3071 * t560;
    let t30637 = t28335 * t113;
    let t30643 = t28390 * t113;
    let t30691 = t6212 * t2892;
    let t30792 = t146 * t5094 * t978;
    let t30856 = t3053 * t481;
    (t30320, t30370, t30428, t30468, t30628, t30637, t30643, t30691, t30792, t30856)
}
