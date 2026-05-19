//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1177/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1177<F: Float>(t10760: F, t20298: F, t24166: F, t261: F, t3299: F, t7386: F, t11720: F, t19872: F, t26274: F, t6093: F, t38088: F, t38093: F, t40114: F, t40117: F, t40120: F, t40123: F, t40128: F, t40131: F) -> F {
    let t40134 = t20298 * t10760 * t24166;
    let t40137 = t3299 * t261 * t7386;
    let t40139 = t19872 * t11720;
    let t40142 = t6093 * t10760 * t26274;
    let t40144 = -F::cast_from(0.43663693315433241792e-2_f64) * t40114 - F::cast_from(0.65495539973149862688e-2_f64) * t40117 - F::cast_from(0.65495539973149862688e-2_f64) * t40120 - F::cast_from(0.26198215989259945075e-1_f64) * t40123 - F::cast_from(0.23287303101564395623e-1_f64) * t38088 - F::cast_from(0.23287303101564395623e-1_f64) * t38093 - F::cast_from(0.21831846657716620896e-2_f64) * t40128 + F::cast_from(0.93149212406257582491e-1_f64) * t40131 + F::cast_from(0.43663693315433241792e-2_f64) * t40134 - F::cast_from(0.42377972951376424087e0_f64) * t40137 - F::cast_from(0.13099107994629972538e-1_f64) * t40139 - F::cast_from(0.13099107994629972538e-1_f64) * t40142;
    t40144
}
