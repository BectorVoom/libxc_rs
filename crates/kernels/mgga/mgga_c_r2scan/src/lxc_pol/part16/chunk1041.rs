//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1041/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1041<F: Float>(t10776: F, t3308: F, t9139: F, t10772: F, t9143: F, t39421: F, t43026: F, t43029: F, t43032: F, t43034: F, t43037: F, t43040: F, t43042: F, t43045: F, t43048: F, t3295: F, t9526: F) -> (F, F) {
    let t43051 = t10776 * t3308 * t9139;
    let t43054 = t10772 * t3308 * t9143;
    let t43056 = -0.69345773920434148507e0 * t43026 - 0.43341108700271342816e-1 * t43029 - t39421 - 0.43341108700271342816e-1 * t43032 + 0.86682217400542685632e-1 * t43034 + 0.86682217400542685632e-1 * t43037 + 0.86682217400542685632e-1 * t43040 + 0.2600466522016280569e0 * t43042 + 0.2600466522016280569e0 * t43045 + 0.2600466522016280569e0 * t43048 + 0.43341108700271342816e-1 * t43051 + 0.13002332610081402845e0 * t43054;
    let t43057 = t3295 * t9526;
    (t43056, t43057)
}
