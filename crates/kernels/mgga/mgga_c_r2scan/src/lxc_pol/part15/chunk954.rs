//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 954/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk954<F: Float>(t37414: F, t10831: F, t1102: F, t3457: F, t2312: F, t597: F, t10680: F, t10682: F, t10645: F, t10646: F, t550: F, t3447: F, t58: F, t166: F, t874: F, t424: F) -> (F, F, F, F, F, F, F, F) {
    let t37415 = 0.91462949374725084942e-3 * t37414;
    let t37419 = t1102 * t10831 * t3457;
    let t37421 = t597 * t2312;
    let t37423 = t10680 * t10682 * t37421;
    let t37426 = t10645 * t10646 * t550;
    let t37427 = t3447 * t58;
    let t37428 = t166 * t874;
    let t37431 = t37426 * t37427 * t424 * t37428;
    (t37415, t37419, t37421, t37423, t37426, t37427, t37428, t37431)
}
