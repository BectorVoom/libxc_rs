//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1177/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1177(t10760: f64, t20298: f64, t24166: f64, t261: f64, t3299: f64, t7386: f64, t11720: f64, t19872: f64, t26274: f64, t6093: f64, t38088: f64, t38093: f64, t40114: f64, t40117: f64, t40120: f64, t40123: f64, t40128: f64, t40131: f64) -> f64 {
    let t40134 = t20298 * t10760 * t24166;
    let t40137 = t3299 * t261 * t7386;
    let t40139 = t19872 * t11720;
    let t40142 = t6093 * t10760 * t26274;
    let t40144 = -0.43663693315433241792e-2_f64 * t40114 - 0.65495539973149862688e-2_f64 * t40117 - 0.65495539973149862688e-2_f64 * t40120 - 0.26198215989259945075e-1_f64 * t40123 - 0.23287303101564395623e-1_f64 * t38088 - 0.23287303101564395623e-1_f64 * t38093 - 0.21831846657716620896e-2_f64 * t40128 + 0.93149212406257582491e-1_f64 * t40131 + 0.43663693315433241792e-2_f64 * t40134 - 0.42377972951376424087e0_f64 * t40137 - 0.13099107994629972538e-1_f64 * t40139 - 0.13099107994629972538e-1_f64 * t40142;
    t40144
}
