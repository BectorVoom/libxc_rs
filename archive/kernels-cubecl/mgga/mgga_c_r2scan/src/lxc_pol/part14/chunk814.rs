//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 814/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk814<F: Float>(t6343: F, t910: F, t551: F, t566: F, t6512: F, t924: F, t552: F, t7088: F, t133: F, t255: F, t2832: F, t546: F) -> (F, F, F, F, F, F, F) {
    let t7233 = t6343 * t910;
    let t7234 = t551 * t7233;
    let t7235 = t566 * t7234;
    let t7237 = t6512 * t924;
    let t7239 = t552 * t7088;
    let t7240 = t551 * t7239;
    let t7244 = t133 * t2832 * t255;
    let t7245 = t546 * t7244;
    (t7233, t7235, t7237, t7239, t7240, t7244, t7245)
}
