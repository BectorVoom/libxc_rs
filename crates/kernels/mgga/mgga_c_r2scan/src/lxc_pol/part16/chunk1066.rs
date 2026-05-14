//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1066/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1066<F: Float>(t37985: F, t38002: F, t38003: F, t39940: F, t39942: F, t43424: F, t43426: F, t43428: F, t43432: F, t43435: F, t43438: F, t43441: F, t38145: F, t6093: F, t9246: F, t2201: F, t3216: F, t3319: F, t3320: F) -> (F, F, F) {
    let t43443 = -t39940 - 0.86682217400542685632e-1 * t43424 - 0.86682217400542685632e-1 * t43426 + t39942 + 0.64025200389650807209e-1 * t43428 + 0.59512461497092438715e-1 * t37985 - t38002 + 0.16262400898971305031e-3 * t38003 + 0.27439371595564631661e-2 * t43432 - 0.23115257973478049502e0 * t43435 - 0.13869154784086829701e1 * t43438 + 0.46574606203128791245e-1 * t43441;
    let t43447 = t6093 * t38145 * t9246;
    let t43451 = t2201 * t3319 * t3320 * t3216;
    (t43443, t43447, t43451)
}
