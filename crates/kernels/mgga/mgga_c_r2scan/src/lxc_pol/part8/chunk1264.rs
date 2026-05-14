//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1264/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1264<F: Float>(t20594: F, t2687: F, t7619: F, t20090: F, t3186: F, t625: F, t3052: F, t538: F, t6191: F, t6194: F, t3090: F, t6212: F, t20237: F, t6211: F, t1584: F, t9218: F) -> (F, F, F, F, F) {
    let t29155 = t20594 * t2687 * t7619;
    let t29158 = t20090 * t3186 * t625;
    let t29175 = t6191 * t538 * t3052 * t6194;
    let t29177 = t6212 * t3090;
    let t29179 = t20237 * t6211 * t29177;
    let t29181 = t1584 * t9218;
    (t29155, t29158, t29175, t29179, t29181)
}
