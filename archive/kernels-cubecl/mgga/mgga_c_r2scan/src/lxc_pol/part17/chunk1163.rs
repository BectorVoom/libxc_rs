//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1163/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1163<F: Float>(t11805: F, t39378: F, t10772: F, t3308: F, t8833: F, t1577: F, t9547: F, t10781: F, t9258: F, t3295: F, t9536: F, t6362: F, t9543: F) -> (F, F, F, F, F, F) {
    let t43135 = t39378 * t11805;
    let t43138 = t10772 * t3308 * t8833;
    let t43141 = t1577 * t3308 * t9547;
    let t43144 = t10781 * t9258;
    let t43146 = t3295 * t9536;
    let t43149 = t6362 * t3308 * t9543;
    (t43135, t43138, t43141, t43144, t43146, t43149)
}
