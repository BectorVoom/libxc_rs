//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1083/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1083<F: Float>(t10760: F, t2147: F, t30304: F, t3178: F, t545: F, t3300: F, t38150: F, t38153: F, t38165: F, t38166: F, t38170: F, t38176: F, t38177: F, t40242: F, t40244: F, t41770: F) -> (F,) {
    let t43677 = t2147 * t10760 * t30304;
    let t43681 = t545 * t3178;
    let t43682 = t43681 * t3300;
    let t43687 = 0.21831846657716620896e-2 * t43677 + 0.81312004494856525156e-4 * t38150 - 0.28914548798370980346e-3 * t38153 - 0.43341108700271342816e-1 * t43682 - t40242 - t40244 + t38165 + 0.42377972951376424087e0 * t38166 + 0.22511059664845582436e0 * t38170 + t38176 - 0.32927245914677557994e-1 * t38177 - t41770;
    (t43687,)
}
