//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1210/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1210(t38145: f64, t6085: f64, t9242: f64, t37985: f64, t38002: f64, t38003: f64, t39940: f64, t39942: f64, t43424: f64, t43426: f64, t43428: f64, t43432: f64, t43435: f64, t43438: f64) -> f64 {
    let t43441 = t6085 * t38145 * t9242;
    let t43443 = -t39940 - 0.86682217400542685632e-1_f64 * t43424 - 0.86682217400542685632e-1_f64 * t43426 + t39942 + 0.64025200389650807209e-1_f64 * t43428 + 0.59512461497092438715e-1_f64 * t37985 - t38002 + 0.16262400898971305031e-3_f64 * t38003 + 0.27439371595564631661e-2_f64 * t43432 - 0.23115257973478049502e0_f64 * t43435 - 0.13869154784086829701e1_f64 * t43438 + 0.46574606203128791245e-1_f64 * t43441;
    t43443
}
