//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1113/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1113<F: Float>(t38303: F, t39116: F, t39117: F, t39121: F, t40659: F, t40672: F, t43939: F, t43943: F, t45058: F, t45060: F, t45066: F, t45068: F, t45070: F, t45073: F, t45078: F, t1115: F, t3060: F, t36967: F) -> (F, F) {
    let t45079 = t45058 - t45060 - t39116 + 0.13680077012009379e-5 * t40659 - 0.30487649791575028312e-3 * t43939 + 0.43368970657079495308e-4 * t43943 - t45066 - t45068 + t45070 - t45073 - 0.14088275218353950416e-1 * t40672 - t39117 - 0.1440846329149835838e-2 * t38303 + t39121 - t45078;
    let t45081 = t36967 * t1115 * t3060;
    (t45079, t45081)
}
