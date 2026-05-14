//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 967/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk967<F: Float>(t10968: F, t6262: F, t6855: F, t10930: F, t158: F, t2304: F, t2317: F, t3434: F, t357: F, t6854: F, t862: F, t1615: F, t875: F, t269: F, t3438: F, t597: F) -> (F, F, F, F, F) {
    let t38228 = t6855 * t6262 * t10968;
    let t38233 = t3434 * t2304 * t2317 * t158 * t10930;
    let t38240 = t862 * t357 * t6854;
    let t38241 = t1615 * t875;
    let t38244 = t38240 * t38241 * t3438 * t269;
    let t38248 = t6855 * t38241;
    let t38249 = t597 * t269;
    (t38228, t38233, t38244, t38248, t38249)
}
