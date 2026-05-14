//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 800/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk800<F: Float>(t6086: F, t9246: F, t6093: F, t6063: F, t2155: F, t2124: F, t7994: F, t921: F, t2531: F, t2545: F, t7977: F, t360: F, t3177: F, t784: F, t783: F, t788: F) -> (F, F, F, F, F, F, F) {
    let t9247 = t6086 * t9246;
    let t9248 = t6093 * t9247;
    let t9250 = t6063 * t9246;
    let t9251 = t2155 * t9250;
    let t9254 = t2124 * t7994 * t921;
    let t9258 = t2124 * t2545 * t2531;
    let t9261 = t7977 * t921;
    let t9262 = t360 * t9261;
    let t9268 = t3177 * t784;
    let t9270 = t783 * t9268 * t788;
    (t9248, t9251, t9254, t9258, t9261, t9262, t9270)
}
