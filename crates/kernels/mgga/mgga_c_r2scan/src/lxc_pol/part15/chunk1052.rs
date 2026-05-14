//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1052/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1052<F: Float>(t11720: F, t19872: F, t10760: F, t26274: F, t6093: F, t38088: F, t38093: F, t40114: F, t40117: F, t40120: F, t40123: F, t40128: F, t40131: F, t40134: F, t40137: F, t10823: F, t7601: F) -> (F, F) {
    let t40139 = t19872 * t11720;
    let t40142 = t6093 * t10760 * t26274;
    let t40144 = -0.43663693315433241792e-2 * t40114 - 0.65495539973149862688e-2 * t40117 - 0.65495539973149862688e-2 * t40120 - 0.26198215989259945075e-1 * t40123 - 0.23287303101564395623e-1 * t38088 - 0.23287303101564395623e-1 * t38093 - 0.21831846657716620896e-2 * t40128 + 0.93149212406257582491e-1 * t40131 + 0.43663693315433241792e-2 * t40134 - 0.42377972951376424087e0 * t40137 - 0.13099107994629972538e-1 * t40139 - 0.13099107994629972538e-1 * t40142;
    let t40145 = t7601 * t10823;
    (t40144, t40145)
}
