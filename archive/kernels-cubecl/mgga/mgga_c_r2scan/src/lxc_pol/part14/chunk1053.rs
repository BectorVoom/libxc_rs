//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1053/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1053<F: Float>(t10831: F, t1102: F, t3457: F, t2312: F, t597: F, t10680: F, t10682: F, t10645: F, t10646: F, t550: F, t3447: F, t58: F) -> (F, F, F, F, F) {
    let t37419 = t1102 * t10831 * t3457;
    let t37421 = t597 * t2312;
    let t37423 = t10680 * t10682 * t37421;
    let t37426 = t10645 * t10646 * t550;
    let t37427 = t3447 * t58;
    (t37419, t37421, t37423, t37426, t37427)
}
