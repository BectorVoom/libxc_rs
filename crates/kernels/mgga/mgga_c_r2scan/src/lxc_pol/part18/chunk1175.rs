//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1175/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1175<F: Float>(t12479: F, t37641: F, t10772: F, t3308: F, t9261: F, t9135: F, t10776: F, t9139: F, t9143: F, t39421: F, t43026: F, t43029: F, t43032: F, t43034: F, t43037: F, t43040: F) -> F {
    let t43042 = t37641 * t12479;
    let t43045 = t10772 * t3308 * t9261;
    let t43048 = t10772 * t3308 * t9135;
    let t43051 = t10776 * t3308 * t9139;
    let t43054 = t10772 * t3308 * t9143;
    let t43056 = -F::new(0.69345773920434148507e0) * t43026 - F::new(0.43341108700271342816e-1) * t43029 - t39421 - F::new(0.43341108700271342816e-1) * t43032 + F::new(0.86682217400542685632e-1) * t43034 + F::new(0.86682217400542685632e-1) * t43037 + F::new(0.86682217400542685632e-1) * t43040 + F::new(0.2600466522016280569e0) * t43042 + F::new(0.2600466522016280569e0) * t43045 + F::new(0.2600466522016280569e0) * t43048 + F::new(0.43341108700271342816e-1) * t43051 + F::new(0.13002332610081402845e0) * t43054;
    t43056
}
