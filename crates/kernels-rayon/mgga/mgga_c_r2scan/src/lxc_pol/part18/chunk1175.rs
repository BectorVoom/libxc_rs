//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1175/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1175(t12479: f64, t37641: f64, t10772: f64, t3308: f64, t9261: f64, t9135: f64, t10776: f64, t9139: f64, t9143: f64, t39421: f64, t43026: f64, t43029: f64, t43032: f64, t43034: f64, t43037: f64, t43040: f64) -> f64 {
    let t43042 = t37641 * t12479;
    let t43045 = t10772 * t3308 * t9261;
    let t43048 = t10772 * t3308 * t9135;
    let t43051 = t10776 * t3308 * t9139;
    let t43054 = t10772 * t3308 * t9143;
    let t43056 = -0.69345773920434148507e0_f64 * t43026 - 0.43341108700271342816e-1_f64 * t43029 - t39421 - 0.43341108700271342816e-1_f64 * t43032 + 0.86682217400542685632e-1_f64 * t43034 + 0.86682217400542685632e-1_f64 * t43037 + 0.86682217400542685632e-1_f64 * t43040 + 0.2600466522016280569e0_f64 * t43042 + 0.2600466522016280569e0_f64 * t43045 + 0.2600466522016280569e0_f64 * t43048 + 0.43341108700271342816e-1_f64 * t43051 + 0.13002332610081402845e0_f64 * t43054;
    t43056
}
