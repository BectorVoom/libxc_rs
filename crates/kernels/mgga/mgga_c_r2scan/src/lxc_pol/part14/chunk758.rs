//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 758/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk758<F: Float>(t566: F, t7234: F, t6512: F, t924: F, t552: F, t7088: F, t551: F, t133: F, t255: F, t2832: F, t546: F, t565: F, t6212: F, t938: F, t6211: F, t6475: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7235 = t566 * t7234;
    let t7237 = t6512 * t924;
    let t7239 = t552 * t7088;
    let t7240 = t551 * t7239;
    let t7244 = t133 * t2832 * t255;
    let t7245 = t546 * t7244;
    let t7250 = t565 * t7244;
    let t7257 = t6212 * t938;
    let t7258 = t6211 * t7257;
    let t7259 = t6475 * t7258;
    (t7235, t7237, t7239, t7240, t7245, t7250, t7257, t7258, t7259)
}
