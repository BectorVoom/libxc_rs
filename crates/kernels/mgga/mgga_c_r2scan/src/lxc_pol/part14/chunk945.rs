//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 945/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk945<F: Float>(t10872: F, t3300: F, t2253: F, t261: F, t3299: F, t2206: F, t774: F, t146: F) -> (F, F, F, F, F) {
    let t10873 = t10872 * t3300;
    let t10875 = t261 * t2253;
    let t10876 = t3299 * t10875;
    let t10878 = t2206 * t774;
    let t10879 = t146 * t10878;
    (t10873, t10875, t10876, t10878, t10879)
}
