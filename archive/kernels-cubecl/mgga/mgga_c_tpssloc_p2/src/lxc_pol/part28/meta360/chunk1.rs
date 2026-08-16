//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1346/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1346<F: Float>(t13532: F, t2826: F, t136: F, t10216: F, t1409: F, t2244: F, t10304: F, t2775: F, t3966: F, t607: F, t908: F, t2250: F, t4342: F) -> (F, F, F, F, F, F) {
    let t13533 = t2826 * t13532;
    let t13534 = t136 * t13533;
    let t13536 = t10216 * t1409;
    let t13537 = t13536 * t2244;
    let t13538 = t10304 * t13537;
    let t13539 = t136 * t13538;
    let t13541 = t2775 * t3966;
    let t13542 = t13541 * t607;
    let t13543 = t908 * t13542;
    let t13544 = t136 * t13543;
    let t13546 = t4342 * t2250;
    (t13534, t13537, t13539, t13542, t13544, t13546)
}
