//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1407/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1407<F: Float>(t6396: F, t91100: F, t20450: F, t22833: F, t6390: F, t91388: F, t1339: F, t1824: F, t22827: F, t550: F, t6347: F, t1799: F, t6414: F) -> (F, F, F, F, F) {
    let t107145 = t91100 * t6396;
    let t107147 = t22833 * t20450;
    let t107151 = t91388 * t6390;
    let t107159 = t22827 * t1339 * t6347 * t1824 * t550;
    let t107164 = t22827 * t1339 * t1799 * t6414 * t550;
    (t107145, t107147, t107151, t107159, t107164)
}
