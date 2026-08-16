//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1157/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1157(t38145: f64, t6535: f64, t8089: f64, t10760: f64, t20298: f64, t24166: f64, t261: f64, t3299: f64, t7386: f64, t11720: f64, t19872: f64, t26274: f64, t6093: f64) -> (f64, f64, f64, f64, f64) {
    let t40131 = t6535 * t38145 * t8089;
    let t40134 = t20298 * t10760 * t24166;
    let t40137 = t3299 * t261 * t7386;
    let t40139 = t19872 * t11720;
    let t40142 = t6093 * t10760 * t26274;
    (t40131, t40134, t40137, t40139, t40142)
}
