//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 838/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk838<F: Float>(t1561: F, t3347: F, t3270: F, t3348: F, t357: F, t862: F, t255: F, t868: F, t258: F) -> (F, F, F, F, F) {
    let t10615 = t1561 * t3347;
    let t10619 = t3270 * t3348;
    let t10645 = t862 * t357;
    let t10646 = t868 * t255;
    let t10647 = t10646 * t258;
    (t10615, t10619, t10645, t10646, t10647)
}
