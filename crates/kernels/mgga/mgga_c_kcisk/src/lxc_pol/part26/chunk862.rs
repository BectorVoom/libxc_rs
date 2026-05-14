//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 862/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk862<F: Float>(t12694: F, t12701: F, t12703: F, t12706: F, t12708: F, t12710: F, t12714: F, t12717: F, t12771: F, t12774: F, t12776: F, t12779: F, t12782: F, t12811: F, t15744: F, t222: F, t3276: F) -> (F, F) {
    let t15759 = 0.1125e1 * t12694 + 0.2428125e0 * t12701 - 0.3375e1 * t12703 + 0.12140625e0 * t12706 - 0.5625e0 * t12708 - 0.97125e0 * t12710 - 0.1125e1 * t12714 + 0.97125e0 * t12717 + 0.4046875e-1 * t12771 - 0.485625e0 * t12774 + 0.12140625e0 * t12776 - 0.1875e0 * t12779 + 0.1125e1 * t12782 - 0.4046875e-1 * t12811;
    let t15760 = t15744 + t15759;
    let t15772 = 1.0 / t3276 / t222;
    (t15760, t15772)
}
