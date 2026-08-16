//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1065/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1065<F: Float>(t10707: F, t2195: F, t2183: F, t20544: F, t252: F, t277: F, t6077: F, t6261: F, t783: F, t2150: F, t37470: F, t574: F) -> (F, F, F, F) {
    let t37582 = t2195 * t10707;
    let t37586 = t2183 * t10707;
    let t37599 = t783 * t252 * t20544 / t6077 / t6261 * t277;
    let t37616 = t574 * t37470 * t2150;
    (t37582, t37586, t37599, t37616)
}
