//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1971/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1971<F: Float>(t14726: F, t15395: F, t11589: F, t4904: F, t3447: F, t11588: F, t461: F) -> (F, F, F, F) {
    let t15396 = t15395 * t14726;
    let t15399 = t11589 * t4904;
    let t15401 = F::cast_from(0.18518518518518518518e-3_f64) * t3447 * t15399;
    let t15402 = t11588 * t461;
    (t15396, t15399, t15401, t15402)
}
